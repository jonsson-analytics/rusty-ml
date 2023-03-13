use core::panic;
use std::collections::HashMap;

use crate::syntax::surface::{
  self,
  types,
};
use crate::transform_into::TransformInto;

#[derive(Debug, Clone, Default)]
pub struct Context
{
  stack: Vec<(surface::Identifier, types::Variable)>,
  constraints: Vec<types::Constraint>,
  assumptions: HashMap<types::Variable, types::Type>,
  free_name: usize,
}

impl Context
{
  fn free_name(&mut self) -> types::Variable
  {
    let free_name = self.free_name;
    self.free_name += 1;
    types::Variable::Unnamed(free_name)
  }

  fn lookup(
    &self,
    name: &str,
  ) -> Option<types::Type>
  {
    self
      .stack
      .iter()
      .rev()
      .find_map(|(binding, typ)| match binding.name == name {
        | true => Some(typ.clone().into()),
        | false => None,
      })
  }

  fn solve_constraints(&mut self)
  {
    while let Some(constraint) = dbg!(self.constraints.pop()) {
      dbg!(&self);
      match constraint {
        | types::Constraint::Equivalent(equivalent) => equivalent.solve(self),
      }
    }
  }
}

pub trait Resolve
{
  fn resolve(
    &self,
    context: &Context,
  ) -> Self;
}

impl Resolve for types::Type
{
  fn resolve(
    &self,
    context: &Context,
  ) -> Self
  {
    match self {
      | types::Type::Abstraction(abstraction) => types::Abstraction {
        parameter_type: abstraction
          .parameter_type
          .resolve(context),
        return_type: abstraction.return_type.resolve(context),
      }
      .into(),
      | types::Type::Variable(variable) =>
        match context.assumptions.get(variable) {
          | Some(resolved) => resolved.resolve(context),
          | None => self.clone(),
        },
      | concrete @ types::Type::Concrete(_) => concrete.clone(),
    }
  }
}

pub trait Solve
{
  fn solve(
    &self,
    context: &mut Context,
  );
}

impl Solve for types::Equivalent
{
  fn solve(
    &self,
    context: &mut Context,
  )
  {
    match self {
      | types::Equivalent {
        left: types::Type::Concrete(left),
        right: types::Type::Concrete(right),
      } => match () {
        | _ if left == right => {
          panic!("Type mismatch: {} != {}", left.name, right.name);
        },
        | _ => (),
      },
      | types::Equivalent {
        left: types::Type::Abstraction(_),
        right: types::Type::Concrete(right),
      } => {
        panic!("Type mismatch: {} is not a function", right.name);
      },
      | types::Equivalent {
        left: types::Type::Concrete(left),
        right: types::Type::Abstraction(_),
      } => {
        panic!("Type mismatch: {} is not a function", left.name);
      },
      | types::Equivalent {
        left: types::Type::Abstraction(left),
        right: types::Type::Abstraction(right),
      } => context.constraints.extend([
        types::Equivalent {
          left: left.parameter_type.clone(),
          right: right.parameter_type.clone(),
        }
        .into(),
        types::Equivalent {
          left: left.return_type.clone(),
          right: right.return_type.clone(),
        }
        .into(),
      ]),
      | types::Equivalent {
        left: types::Type::Variable(left),
        right,
      } => match dbg!(context.assumptions.get(left)) {
        | Some(left) => {
          context.constraints.push(
            types::Equivalent {
              left: left.clone(),
              right: right.clone(),
            }
            .into(),
          );
        },
        | None => {
          debug_assert_eq!(
            context
              .assumptions
              .insert(left.clone(), right.clone()),
            None
          );
        },
      },
      | types::Equivalent {
        right: right @ types::Type::Variable(_),
        left,
      } => context.constraints.push(
        types::Equivalent {
          left: right.clone(),
          right: left.clone(),
        }
        .into(),
      ),
    }
  }
}

pub trait InferType<'a>
{
  fn infer_type(
    &self,
    context: &'a mut Context,
  ) -> types::Type;
}

impl<'a, Representation> InferType<'a> for Representation
where
  Representation: TransformInto<types::Type, Context<'a> = &'a mut Context>,
{
  fn infer_type(
    &self,
    context: &'a mut Context,
  ) -> types::Type
  {
    self.transform(context)
  }
}

impl TransformInto<types::Type> for surface::Expression
{
  type Context<'a> = &'a mut Context;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> types::Type
  {
    match self {
      | surface::Expression::Literal(literal) => literal.infer_type(context),
      | surface::Expression::Identifier(identifier) =>
        identifier.infer_type(context),
      | surface::Expression::Abstraction(abstraction) =>
        abstraction.infer_type(context),
      | surface::Expression::Application(application) =>
        application.infer_type(context),
    }
  }
}

impl TransformInto<types::Type> for surface::Application
{
  type Context<'a> = &'a mut Context;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> types::Type
  {
    let arguments = self
      .arguments
      .iter()
      .map(|argument| argument.infer_type(context))
      .collect::<Vec<_>>();

    let mut actual_abstraction_type = self.abstraction.infer_type(context);
    for argument_type in arguments {
      let return_type: types::Type = context.free_name().into();
      let assumed_abstraction_type =
        types::Type::abstraction(argument_type, return_type.clone());
      context.constraints.push(
        types::Equivalent {
          left: actual_abstraction_type,
          right: assumed_abstraction_type,
        }
        .into(),
      );
      actual_abstraction_type = return_type;
    }
    actual_abstraction_type
  }
}

impl TransformInto<types::Type> for surface::Abstraction
{
  type Context<'a> = &'a mut Context;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> types::Type
  {
    let parameters = self
      .parameters
      .iter()
      .map(|parameter| (parameter.clone(), context.free_name()))
      .collect::<Vec<_>>();

    context
      .stack
      .extend(parameters.iter().cloned());

    let mut return_type = self.body.infer_type(context);
    for (_, parameter_type) in parameters.iter().rev() {
      return_type =
        types::Type::abstraction(parameter_type.clone().into(), return_type);
      context.stack.pop();
    }
    return_type
  }
}

impl TransformInto<types::Type> for surface::Identifier
{
  type Context<'a> = &'a mut Context;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> types::Type
  {
    context
      .lookup(self.name.as_str())
      .unwrap_or_else(|| context.free_name().into())
  }
}

impl TransformInto<types::Type> for surface::Literal
{
  type Context<'a> = &'a mut Context;

  fn transform(
    &self,
    _context: Self::Context<'_>,
  ) -> types::Type
  {
    match self {
      | surface::Literal::String(_) =>
        surface::Identifier::new("String").into(),
      | surface::Literal::Numeric(_) =>
        surface::Identifier::new("Numeric").into(),
      | surface::Literal::Boolean(_) =>
        surface::Identifier::new("Boolean").into(),
    }
  }
}

#[cfg(test)]
mod spec
{
  use super::*;

  macro_rules! assert_type {
    ($name:ident; $input:literal resolves to $expected:expr) => {
      #[test]
      fn $name()
      {
        use crate::frontend::{
          ExpressionParser,
          Lexer,
          WithBacktracking,
        };
        let mut lexer = Lexer::from_str($input).with_backtracking();
        let expression = lexer.expect_expression().unwrap();
        let mut context = Context::default();
        let typ = dbg!(expression.infer_type(&mut context));
        dbg!(&context);
        context.solve_constraints();
        dbg!(&context);
        let typ = dbg!(typ.resolve(&context));
        pretty_assertions::assert_eq!(typ, $expected);
      }
    };
  }

  assert_type!(string_literal_resolves_to_true;
    "`foo`" resolves to surface::Identifier::new("String").into()
  );
  assert_type!(numeric_literal_resolves_to_numeric;
    "10" resolves to surface::Identifier::new("Numeric").into()
  );
  assert_type!(true_resolves_to_boolean;
    "true" resolves to surface::Identifier::new("Boolean").into()
  );
  assert_type!(false_resolves_to_boolean;
    "false" resolves to surface::Identifier::new("Boolean").into()
  );
  assert_type!(abstraction_one_parameter_returns_first;
      "(fun x -> x)" resolves to types::Type::abstraction(
      types::Variable::Unnamed(0).into(),
      types::Variable::Unnamed(0).into(),
    )
  );
  assert_type!(abstraction_two_parameters_returns_first;
      "(fun x y -> x)" resolves to types::Type::abstraction(
      types::Variable::Unnamed(0).into(),
      types::Type::abstraction(
        types::Variable::Unnamed(1).into(),
        types::Variable::Unnamed(0).into(),
      ),
    )
  );
  assert_type!(abstraction_two_parameters_returns_second;
    "(fun x y -> y)" resolves to types::Type::abstraction(
      types::Variable::Unnamed(0).into(),
      types::Type::abstraction(
        types::Variable::Unnamed(1).into(),
        types::Variable::Unnamed(1).into(),
      ),
    )
  );
  assert_type!(abstraction_one_parameter_fully_applied_returns_first;
    "(fun x -> x) 10" resolves to surface::Identifier::new("Numeric").into()
  );
  assert_type!(abstraction_two_parameters_fully_applied_returns_first;
    "(fun x y -> x) 10 `foo`" resolves to surface::Identifier::new("Numeric").into()
  );
  assert_type!(abstraction_two_parameters_fully_applied_returns_second;
    "(fun x y -> y) 10 `foo`" resolves to surface::Identifier::new("String").into()
  );
}

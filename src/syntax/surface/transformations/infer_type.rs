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
  free_name: usize,
}

impl Context
{
  pub fn free_name(&mut self) -> types::Variable
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
        types::Type::monomorphic(surface::Identifier::new("String").into()),
      | surface::Literal::Numeric(_) =>
        types::Type::monomorphic(surface::Identifier::new("Numeric").into()),
      | surface::Literal::Boolean(_) =>
        types::Type::monomorphic(surface::Identifier::new("Boolean").into()),
    }
  }
}

#[cfg(test)]
mod spec
{
  use pretty_assertions::assert_eq;

  use super::*;
  use crate::frontend::{
    ExpressionParser,
    Lexer,
    WithBacktracking,
  };

  #[test]
  fn test()
  {
    let mut lexer = Lexer::from_str("(fun x -> x) 10").with_backtracking();
    let expression = lexer.expect_expression().unwrap();
    let mut context = Context::default();
    dbg!(expression.infer_type(&mut context));
    dbg!(context);
    todo!()
  }
}

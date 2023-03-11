use super::LargestFreeVariable;
use crate::syntax::debrujin;
use crate::transform_into::TransformInto;

pub trait Evaluate<'a>
{
  fn evaluate(
    &self,
    context: &'a mut Context,
  ) -> Value;
}

impl<'a, Representation> Evaluate<'a> for Representation
where
  Representation: TransformInto<Value, Context<'a> = &'a mut Context>,
{
  fn evaluate(
    &self,
    context: &'a mut Context,
  ) -> Value
  {
    self.transform(context)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value
{
  String(String),
  Bool(bool),
  F64(f64),
  Closure
  {
    stack: Vec<Value>,
    body: debrujin::Expression,
  },
}

pub struct Context
{
  stack: Vec<Value>,
}

impl Context
{
  pub fn evaluate<'a, Representation>(
    &'a mut self,
    representation: Representation,
  ) -> Value
  where
    Representation: TransformInto<Value, Context<'a> = &'a mut Context>,
  {
    representation.transform(self)
  }

  pub fn load<Result>(
    &mut self,
    closure: &[Value],
    with_closure: impl FnOnce(&mut Self) -> Result,
  ) -> Result
  {
    self
      .stack
      .extend(closure.iter().cloned());
    let result = with_closure(self);
    for _ in 0 .. closure.len() {
      self.stack.pop();
    }
    return result
  }

  pub fn lookup(
    &self,
    name: usize,
  ) -> Option<Value>
  {
    // todo: this is a prototype, it's not efficient
    self
      .stack
      .iter()
      .rev()
      .skip(name)
      .next()
      .cloned()
  }

  pub fn capture(
    &self,
    until: usize,
  ) -> Vec<Value>
  {
    self
      .stack
      .iter()
      .rev()
      .take(until)
      .rev()
      .cloned()
      .collect()
  }
}

impl Default for Context
{
  fn default() -> Self
  {
    Self {
      stack: Vec::new(),
    }
  }
}

impl TransformInto<Value> for debrujin::Expression
{
  type Context<'a> = &'a mut Context;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> Value
  {
    match self {
      | debrujin::Expression::Literal(literal) => literal.transform(context),
      | debrujin::Expression::Identifier(identifier) =>
        identifier.transform(context),
      | debrujin::Expression::Abstraction(abstraction) =>
        abstraction.transform(context),
      | debrujin::Expression::Application(application) =>
        application.transform(context),
    }
  }
}

impl TransformInto<Value> for debrujin::Literal
{
  type Context<'a> = &'a mut Context;

  fn transform(
    &self,
    _context: Self::Context<'_>,
  ) -> Value
  {
    match self {
      | debrujin::Literal::String(value) => Value::String(value.clone()),
      | debrujin::Literal::Number(value) => Value::F64(*value),
      | debrujin::Literal::Boolean(value) => Value::Bool(*value),
    }
  }
}

#[cfg(test)]
mod literals
{
  use super::*;

  #[test]
  fn string()
  {
    let literal = debrujin::Literal::String("hello".into());
    let mut context = Context::default();
    assert_eq!(context.evaluate(literal), Value::String("hello".into()));
  }

  #[test]
  fn number()
  {
    let literal = debrujin::Literal::Number(3.14);
    let mut context = Context::default();
    assert_eq!(context.evaluate(literal), Value::F64(3.14));
  }

  #[test]
  fn boolean()
  {
    let literal = debrujin::Literal::Boolean(true);
    let mut context = Context::default();
    assert_eq!(context.evaluate(literal), Value::Bool(true));
  }
}

impl TransformInto<Value> for debrujin::Identifier
{
  type Context<'a> = &'a mut Context;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> Value
  {
    context
      .lookup(self.name)
      .expect(format!("unbound identifier: {}", self.name).as_str())
  }
}

#[cfg(test)]
mod identifiers
{
  use super::*;

  #[test]
  fn bound()
  {
    let identifier = debrujin::Identifier {
      name: 0,
    };
    let mut context = Context {
      stack: vec![Value::String("hello".into())],
    };
    assert_eq!(context.evaluate(identifier), Value::String("hello".into()),);
  }

  #[test]
  #[should_panic = "unbound identifier: 0"]
  fn unbound()
  {
    let identifier = debrujin::Identifier {
      name: 0,
    };
    let mut context = Context::default();
    assert_eq!(context.evaluate(identifier), Value::String("hello".into()),);
  }
}

impl TransformInto<Value> for debrujin::Abstraction
{
  type Context<'a> = &'a mut Context;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> Value
  {
    // todo: current implementation is not efficient
    Value::Closure {
      stack: context.capture(self.body.largest_free_variable(1)),
      body: self.body.clone(),
    }
  }
}

#[cfg(test)]
mod abstractions
{
  use super::*;

  #[test]
  fn own_argument()
  {
    let body: debrujin::Expression = debrujin::Identifier {
      name: 0,
    }
    .into();
    let abstraction = debrujin::Abstraction {
      body: body.clone(),
    };
    let mut context = Context::default();
    assert_eq!(context.evaluate(abstraction), Value::Closure {
      stack: vec![],
      body,
    });
  }

  #[test]
  fn bound_closure()
  {
    let body: debrujin::Expression = debrujin::Identifier {
      name: 1,
    }
    .into();
    let abstraction = debrujin::Abstraction {
      body: body.clone(),
    };
    let mut context = Context {
      stack: vec![Value::String("hello".into())],
    };
    assert_eq!(context.evaluate(abstraction), Value::Closure {
      stack: vec![Value::String("hello".into())],
      body,
    });
  }

  #[test]
  fn unbound_closure()
  {
    let body: debrujin::Expression = debrujin::Identifier {
      name: 1,
    }
    .into();
    let abstraction = debrujin::Abstraction {
      body: body.clone(),
    };
    let mut context = Context::default();
    assert_eq!(context.evaluate(abstraction), Value::Closure {
      stack: vec![],
      body,
    });
  }
}

impl TransformInto<Value> for debrujin::Application
{
  type Context<'a> = &'a mut Context;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> Value
  {
    let abstraction = self.abstraction.transform(&mut *context);
    let argument = self.argument.transform(&mut *context);
    match abstraction {
      | Value::Closure {
        stack,
        body,
      } => context.load(stack.as_slice(), |context| {
        return context.load(&[argument], |context| body.transform(context))
      }),
      | _ => panic!("not a closure"),
    }
  }
}

#[cfg(test)]
mod application
{
  use super::*;

  #[test]
  fn own_argument()
  {
    let abstraction = debrujin::Application {
      abstraction: debrujin::Abstraction {
        body: debrujin::Identifier {
          name: 0,
        }
        .into(),
      }
      .into(),
      argument: debrujin::Literal::String("hello".into()).into(),
    };
    let mut context = Context::default();
    assert_eq!(context.evaluate(abstraction), Value::String("hello".into()));
  }

  #[test]
  fn bound_closure()
  {
    let abstraction = debrujin::Application {
      abstraction: debrujin::Abstraction {
        body: debrujin::Identifier {
          name: 1,
        }
        .into(),
      }
      .into(),
      argument: debrujin::Literal::String("hello".into()).into(),
    };
    let mut context = Context {
      stack: vec![Value::String("foo".into())],
    };
    assert_eq!(context.evaluate(abstraction), Value::String("foo".into()));
  }

  #[test]
  #[should_panic = "unbound identifier: 1"]
  fn unbound_closure()
  {
    let abstraction = debrujin::Application {
      abstraction: debrujin::Abstraction {
        body: debrujin::Identifier {
          name: 1,
        }
        .into(),
      }
      .into(),
      argument: debrujin::Literal::String("hello".into()).into(),
    };
    let mut context = Context::default();
    assert_eq!(context.evaluate(abstraction), Value::String("foo".into()));
  }
}

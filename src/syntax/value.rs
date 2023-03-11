mod _specification;

use super::debrujin;
use crate::transform_into::TransformInto;

trait StackFrame
{
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

impl TransformInto<Value> for debrujin::Expression
{
  type Environment<'a> = &'a mut Vec<Value>;

  fn encode(
    &self,
    environment: Self::Environment<'_>,
  ) -> Value
  {
    match self {
      | debrujin::Expression::Literal(literal) => literal.encode(()),
      | debrujin::Expression::Identifier(identifier) =>
        identifier.encode(environment),
      | debrujin::Expression::Abstraction(abstraction) =>
        abstraction.encode(environment),
      | debrujin::Expression::Application(application) =>
        application.encode(environment),
    }
  }
}

impl TransformInto<Value> for debrujin::Literal
{
  type Environment<'a> = ();

  fn encode(
    &self,
    _environment: Self::Environment<'_>,
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
    let value = literal.encode(());
    assert_eq!(value, Value::String("hello".into()));
  }

  #[test]
  fn number()
  {
    let literal = debrujin::Literal::Number(3.14);
    let value = literal.encode(());
    assert_eq!(value, Value::F64(3.14));
  }

  #[test]
  fn boolean()
  {
    let literal = debrujin::Literal::Boolean(true);
    let value = literal.encode(());
    assert_eq!(value, Value::Bool(true));
  }
}

impl TransformInto<Value> for debrujin::Identifier
{
  type Environment<'a> = &'a mut Vec<Value>;

  fn encode(
    &self,
    environment: Self::Environment<'_>,
  ) -> Value
  {
    // todo: this is a prototype, it's not efficient
    environment
      .iter()
      .rev()
      .skip(self.name)
      .next()
      .cloned()
      .unwrap_or_else(|| panic!("unbound identifier: {}", self.name))
  }
}

#[cfg(test)]
mod identifiers
{
  use super::*;

  #[test]
  fn bound()
  {
    let mut environment = vec![Value::String("hello".into())];
    let identifier = debrujin::Identifier {
      name: 0,
    };
    let value = identifier.encode(&mut environment);
    assert_eq!(value, Value::String("hello".into()));
  }

  #[test]
  #[should_panic = "unbound identifier: 0"]
  fn unbound()
  {
    let mut environment = vec![];
    let identifier = debrujin::Identifier {
      name: 0,
    };
    let value = identifier.encode(&mut environment);
    assert_eq!(value, Value::String("hello".into()));
  }
}

impl TransformInto<Value> for debrujin::Abstraction
{
  type Environment<'a> = &'a mut Vec<Value>;

  fn encode(
    &self,
    environment: Self::Environment<'_>,
  ) -> Value
  {
    todo!()
  }
}

#[cfg(test)]
mod abstractions
{
  use super::*;

  #[test]
  fn own_argument()
  {
    let mut environment = vec![];
    let body: debrujin::Expression = debrujin::Identifier {
      name: 0,
    }
    .into();
    let abstraction = debrujin::Abstraction {
      body: body.clone(),
    };
    let value = abstraction.encode(&mut environment);
    assert_eq!(value, Value::Closure {
      stack: vec![],
      body,
    });
  }

  #[test]
  fn bound_closure()
  {
    let mut environment = vec![Value::String("hello".into())];
    let body: debrujin::Expression = debrujin::Identifier {
      name: 1,
    }
    .into();
    let abstraction = debrujin::Abstraction {
      body: body.clone(),
    };
    let value = abstraction.encode(&mut environment);
    assert_eq!(value, Value::Closure {
      stack: vec![],
      body,
    });
  }

  #[test]
  #[should_panic = "unbound identifier: 1"]
  fn unbound_closure()
  {
    let mut environment = vec![];
    let body: debrujin::Expression = debrujin::Identifier {
      name: 1,
    }
    .into();
    let abstraction = debrujin::Abstraction {
      body: body.clone(),
    };
    let value = abstraction.encode(&mut environment);
    assert_eq!(value, Value::Closure {
      stack: vec![],
      body,
    });
  }
}

impl TransformInto<Value> for debrujin::Application
{
  type Environment<'a> = &'a mut Vec<Value>;

  fn encode(
    &self,
    environment: Self::Environment<'_>,
  ) -> Value
  {
    todo!()
  }
}

#[cfg(test)]
mod application
{
  use super::*;

  #[test]
  fn own_argument()
  {
    let mut environment = vec![];
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
    let value = abstraction.encode(&mut environment);
    assert_eq!(value, Value::String("hello".into()));
  }

  #[test]
  fn bound_closure()
  {
    let mut environment = vec![Value::String("foo".into())];
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
    let value = abstraction.encode(&mut environment);
    assert_eq!(value, Value::String("foo".into()));
  }

  #[test]
  #[should_panic = "unbound identifier: 1"]
  fn unbound_closure()
  {
    let mut environment = vec![];
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
    let value = abstraction.encode(&mut environment);
    assert_eq!(value, Value::String("foo".into()));
  }
}

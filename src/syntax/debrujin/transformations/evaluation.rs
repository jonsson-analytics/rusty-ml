use super::LargestFreeVariable;
use crate::syntax::{
  debrujin,
  value,
};
use crate::transform_into::TransformInto;

pub struct Evaluation(value::Value);

impl TransformInto<Evaluation> for debrujin::Expression
{
  type Environment<'a> = &'a mut Vec<value::Value>;

  fn encode(
    &self,
    environment: Self::Environment<'_>,
  ) -> Evaluation
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

impl TransformInto<Evaluation> for debrujin::Literal
{
  type Environment<'a> = ();

  fn encode(
    &self,
    _environment: Self::Environment<'_>,
  ) -> Evaluation
  {
    Evaluation(match self {
      | debrujin::Literal::String(value) => value::Value::String(value.clone()),
      | debrujin::Literal::Number(value) => value::Value::F64(*value),
      | debrujin::Literal::Boolean(value) => value::Value::Bool(*value),
    })
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
    let Evaluation(value) = literal.encode(());
    assert_eq!(value, value::Value::String("hello".into()));
  }

  #[test]
  fn number()
  {
    let literal = debrujin::Literal::Number(3.14);
    let Evaluation(value) = literal.encode(());
    assert_eq!(value, value::Value::F64(3.14));
  }

  #[test]
  fn boolean()
  {
    let literal = debrujin::Literal::Boolean(true);
    let Evaluation(value) = literal.encode(());
    assert_eq!(value, value::Value::Bool(true));
  }
}

impl TransformInto<Evaluation> for debrujin::Identifier
{
  type Environment<'a> = &'a mut Vec<value::Value>;

  fn encode(
    &self,
    environment: Self::Environment<'_>,
  ) -> Evaluation
  {
    // todo: this is a prototype, it's not efficient
    Evaluation(
      environment
        .iter()
        .rev()
        .skip(self.name)
        .next()
        .cloned()
        .unwrap_or_else(|| panic!("unbound identifier: {}", self.name)),
    )
  }
}

#[cfg(test)]
mod identifiers
{
  use super::*;

  #[test]
  fn bound()
  {
    let mut environment = vec![value::Value::String("hello".into())];
    let identifier = debrujin::Identifier {
      name: 0,
    };
    let Evaluation(value) = identifier.encode(&mut environment);
    assert_eq!(value, value::Value::String("hello".into()));
  }

  #[test]
  #[should_panic = "unbound identifier: 0"]
  fn unbound()
  {
    let mut environment = vec![];
    let identifier = debrujin::Identifier {
      name: 0,
    };
    let Evaluation(value) = identifier.encode(&mut environment);
    assert_eq!(value, value::Value::String("hello".into()));
  }
}

impl TransformInto<Evaluation> for debrujin::Abstraction
{
  type Environment<'a> = &'a mut Vec<value::Value>;

  fn encode(
    &self,
    environment: Self::Environment<'_>,
  ) -> Evaluation
  {
    let LargestFreeVariable(name) = self.body.encode(0);
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
    let Evaluation(value) = abstraction.encode(&mut environment);
    assert_eq!(value, value::Value::Closure {
      stack: vec![],
      body,
    });
  }

  #[test]
  fn bound_closure()
  {
    let mut environment = vec![value::Value::String("hello".into())];
    let body: debrujin::Expression = debrujin::Identifier {
      name: 1,
    }
    .into();
    let abstraction = debrujin::Abstraction {
      body: body.clone(),
    };
    let Evaluation(value) = abstraction.encode(&mut environment);
    assert_eq!(value, value::Value::Closure {
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
    let Evaluation(value) = abstraction.encode(&mut environment);
    assert_eq!(value, value::Value::Closure {
      stack: vec![],
      body,
    });
  }
}

impl TransformInto<Evaluation> for debrujin::Application
{
  type Environment<'a> = &'a mut Vec<value::Value>;

  fn encode(
    &self,
    environment: Self::Environment<'_>,
  ) -> Evaluation
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
    let Evaluation(value) = abstraction.encode(&mut environment);
    assert_eq!(value, value::Value::String("hello".into()));
  }

  #[test]
  fn bound_closure()
  {
    let mut environment = vec![value::Value::String("foo".into())];
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
    let Evaluation(value) = abstraction.encode(&mut environment);
    assert_eq!(value, value::Value::String("foo".into()));
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
    let Evaluation(value) = abstraction.encode(&mut environment);
    assert_eq!(value, value::Value::String("foo".into()));
  }
}

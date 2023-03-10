use super::transform_into::TransformInto;
use super::{
  common,
  debrujin,
};

trait StackFrame
{
}

pub enum Value
{
  String(String),
  Bool(bool),
  F64(f64),
  Closure
  {
    stack: Vec<Value>,
    abstraction: Box<debrujin::Expression>,
  },
}

impl TransformInto<Value> for common::Literal
{
  type Environment = ();
  type Result<T> = T;

  fn encode(
    &self,
    _environment: &mut Self::Environment,
  ) -> Self::Result<Value>
  {
    match self {
      | common::Literal::String(value) => Value::String(value.clone()),
      | common::Literal::Number(value) => Value::F64(*value),
      | common::Literal::Boolean(value) => Value::Bool(*value),
    }
  }
}

impl TransformInto<Value> for debrujin::Identifier
{
  type Environment = Vec<Value>;
  type Result<T> = std::result::Result<T, ()>;

  fn encode(
    &self,
    environment: &mut Self::Environment,
  ) -> Self::Result<Value>
  {
    todo!()
  }
}

impl TransformInto<Value> for debrujin::Abstraction
{
  type Environment = Vec<Value>;
  type Result<T> = std::result::Result<T, ()>;

  fn encode(
    &self,
    environment: &mut Self::Environment,
  ) -> Self::Result<Value>
  {
    todo!()
  }
}

impl TransformInto<Value> for debrujin::Application
{
  type Environment = Vec<Value>;
  type Result<T> = std::result::Result<T, ()>;

  fn encode(
    &self,
    environment: &mut Self::Environment,
  ) -> Self::Result<Value>
  {
    todo!()
  }
}

impl TransformInto<Value> for debrujin::Expression
{
  type Environment = Vec<Value>;
  type Result<T> = std::result::Result<T, ()>;

  fn encode(
    &self,
    environment: &mut Self::Environment,
  ) -> Self::Result<Value>
  {
    match self {
      | debrujin::Expression::Literal(literal) => Ok(literal.encode(&mut ())),
      | debrujin::Expression::Identifier(identifier) =>
        identifier.encode(environment),
      | debrujin::Expression::Abstraction(abstraction) =>
        abstraction.encode(environment),
      | debrujin::Expression::Application(application) =>
        application.encode(environment),
    }
  }
}

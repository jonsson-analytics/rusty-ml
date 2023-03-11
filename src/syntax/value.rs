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
    abstraction: Box<debrujin::Expression>,
  },
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
      | debrujin::Expression::Identifier(identifier) => todo!(),
      | debrujin::Expression::Abstraction(abstraction) => todo!(),
      | debrujin::Expression::Application(application) =>
        application.encode(environment),
    }
  }
}

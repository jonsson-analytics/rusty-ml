mod _specification;

use super::debrujin::{self, transformations::LargestFreeVariable};
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

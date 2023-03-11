use super::debrujin;

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

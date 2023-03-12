#[derive(Debug, Clone, PartialEq)]
pub enum Literal
{
  String(String),
  Numeric(String),
  Boolean(bool),
}

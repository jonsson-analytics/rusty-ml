#[derive(Debug, Clone, PartialEq)]
pub enum Literal
{
  String(String),
  Number(f64),
  Boolean(bool),
}

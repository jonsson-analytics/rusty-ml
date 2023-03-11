#[derive(Debug, Clone, PartialEq)]
pub enum NodeType
{
  Literal,
  BooleanLiteral,
  Expression,
  Declaration,
  ValBinding,
  DefBinding,
}

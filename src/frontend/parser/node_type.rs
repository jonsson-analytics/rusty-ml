#[derive(Debug, Clone, PartialEq)]
pub enum NodeType
{
  Literal,
  Expression,
  Declaration,
  ValBinding,
  DefBinding,
}

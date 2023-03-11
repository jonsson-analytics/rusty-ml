#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token
{
  Symbol(&'static str),
  Identifier,
  StringLiteral,
  Keyword(&'static str),
  UnclosedComment,
  UnclosedString,
}

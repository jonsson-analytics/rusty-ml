#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token
{
  Symbol(&'static str),
  Keyword(&'static str),
  Identifier,
  StringLiteral,
  NumericLiteral,
  MalformedNumericLiteral,
  UnclosedComment,
  UnclosedString,
}

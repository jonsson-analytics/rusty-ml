#[derive(Debug, PartialEq)]
pub struct Identifier
{
  value: String,
}

#[derive(Debug, PartialEq)]
pub struct StringLiteral
{
  value: String,
}

#[derive(Debug, PartialEq)]
pub struct Comment
{
  value: String,
}

#[derive(Debug, PartialEq)]
pub enum Token
{
  ParenL,
  ParenR,
  BraceL,
  BraceR,
  BracketL,
  BracketR,
  Identifier(Identifier),
  StringLiteral(StringLiteral),
  UnclosedString,
  UnclosedComment,
}

impl Token
{
  pub fn identifier(value: String) -> Self
  {
    Self::Identifier(Identifier {
      value,
    })
  }

  pub fn string(value: String) -> Self
  {
    Self::StringLiteral(StringLiteral {
      value,
    })
  }
}

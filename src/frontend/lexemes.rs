#[derive(Debug, PartialEq)]
pub struct Identifier
{
  pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct StringLiteral
{
  pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct Comment
{
  pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct Keyword
{
  pub value: String,
}

#[derive(Debug, PartialEq)]
pub enum Lexeme
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
  Keyword(Keyword),
}

impl Lexeme
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

  pub fn keyword(value: String) -> Self
  {
    Self::Keyword(Keyword {
      value,
    })
  }
}

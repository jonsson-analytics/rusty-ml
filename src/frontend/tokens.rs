#[derive(Debug, Clone, PartialEq)]
pub enum Token
{
  Symbol(String),
  Identifier,
  StringLiteral,
  Keyword(String),
  UnclosedComment,
  UnclosedString,
}

impl Token
{
  pub fn symbol<IntoString>(value: IntoString) -> Self
  where
    IntoString: Into<String>,
  {
    Self::Symbol(value.into())
  }

  pub fn keyword<IntoString>(value: IntoString) -> Self
  where
    IntoString: Into<String>,
  {
    Self::Keyword(value.into())
  }
}

use super::tokens::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Lexeme
{
  token: Token,
  value: String,
}

impl Lexeme
{
  pub fn unclosed_comment() -> Self
  {
    Self {
      token: Token::UnclosedComment,
      value: "".to_string(),
    }
  }

  pub fn unclosed_string() -> Self
  {
    Self {
      token: Token::UnclosedString,
      value: "".to_string(),
    }
  }

  pub fn symbol(value: &'static str) -> Self
  {
    Self {
      token: Token::Symbol(value),
      value: value.to_string(),
    }
  }

  pub fn identifier<IntoString>(value: IntoString) -> Self
  where
    IntoString: Into<String>,
  {
    Self {
      token: Token::Identifier,
      value: value.into(),
    }
  }

  pub fn string<IntoString>(value: IntoString) -> Self
  where
    IntoString: Into<String>,
  {
    Self {
      token: Token::StringLiteral,
      value: value.into(),
    }
  }

  pub fn keyword(value: &'static str) -> Self
  {
    Self {
      token: Token::Keyword(value),
      value: value.into(),
    }
  }

  pub fn token(&self) -> &Token
  {
    &self.token
  }

  pub fn value(&self) -> &String
  {
    &self.value
  }
}

mod _specification;
mod parse_error;

pub use self::parse_error::ParseError;
use super::syntax::*;
use super::lexemes::Lexeme;
use super::tokens::Token;

pub type Result<T> = std::result::Result<T, ParseError>;


pub struct Parser<Lexer>
{
  lexer: Lexer,
  buffer: Option<Lexeme>,
}

impl<Lexer> Parser<Lexer>
{
  pub fn new(lexer: Lexer) -> Self
  {
    Self {
      lexer,
      buffer: None,
    }
  }
}

impl<'a> Parser<super::lexer::Lexer<'a>>
{
  pub fn from_str(str: &'a str) -> Self
  {
    Self::new(super::lexer::Lexer::from_str(str))
  }
}

impl<Lexer> Iterator for Parser<Lexer>
where
  Lexer: Iterator<Item = Lexeme>,
{
  type Item = TopLevel;

  fn next(&mut self) -> Option<Self::Item>
  {
    todo!()
  }
}

use super::syntax::TopLevel;
use super::tokens::Token;

pub struct Parser<Lexer>
{
  lexer: Lexer,
  buffer: Option<Token>,
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

impl<Lexer> Iterator for Parser<Lexer>
where
  Lexer: Iterator<Item = Token>,
{
  type Item = TopLevel;

  fn next(&mut self) -> Option<Self::Item>
  {
    todo!()
  }
}

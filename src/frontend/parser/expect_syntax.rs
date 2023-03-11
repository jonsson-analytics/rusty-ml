use super::{
  ParseError,
  Result,
};
use crate::frontend::lexemes::Lexeme;
use crate::frontend::tokens::Token;

pub trait ExpectSyntax
where
  Self: Iterator<Item = Lexeme>,
{
  fn expect(
    &mut self,
    expected: Token,
  ) -> Result<Lexeme>
  {
    self
      .next()
      .ok_or_else(|| ParseError::UnexpectedEndOfInput {
        expected,
      })
      .map(|lexeme| match () {
        | _ if lexeme.token() == &expected => Ok(lexeme),
        | _ => Err(ParseError::UnexpectedToken {
          expected,
          actual: lexeme,
        }),
      })
      .flatten()
  }
}

impl<Lexer> ExpectSyntax for Lexer where Lexer: Iterator<Item = Lexeme>
{
}

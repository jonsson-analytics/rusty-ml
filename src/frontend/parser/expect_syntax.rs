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
      .ok_or(ParseError::UnexpectedEndOfInput {
        expected,
      })
      .and_then(|lexeme| match () {
        | _ if lexeme.token() == &expected => Ok(lexeme),
        | _ => Err(ParseError::UnexpectedToken {
          expected,
          actual: lexeme,
        }),
      })
  }
}

impl<Lexer> ExpectSyntax for Lexer where Lexer: Iterator<Item = Lexeme>
{
}

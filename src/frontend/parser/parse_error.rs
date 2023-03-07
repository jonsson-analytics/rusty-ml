use crate::frontend::lexemes::Lexeme;
use crate::frontend::tokens::Token;

pub enum ParseError
{
  UnexpectedToken
  {
    expected: Token,
    actual: Lexeme,
  },
  UnexpectedEndOfInput
  {
    expected: Token,
  },
}

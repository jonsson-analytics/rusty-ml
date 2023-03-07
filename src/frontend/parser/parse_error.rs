use crate::frontend::lexemes::Lexeme;
use crate::frontend::tokens::Token;

#[derive(Debug, Clone, PartialEq)]
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

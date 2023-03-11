use crate::frontend::lexemes::Lexeme;
use crate::frontend::tokens::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError
{
  UnexpectedEndOfInput
  {
    expected: Token,
  },
  UnexpectedToken
  {
    expected: Token,
    actual: Lexeme,
  },
}

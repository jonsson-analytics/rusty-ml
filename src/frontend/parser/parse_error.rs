use crate::frontend::tokens::Token;

pub enum ParseError
{
  UnexpectedToken
  {
    expected: Token,
    actual: Token,
  },
  UnexpectedEndOfInput
  {
    expected: Token,
  },
}

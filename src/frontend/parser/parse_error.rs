use crate::frontend::lexemes::Lexeme;
use crate::frontend::tokens::Token;

use super::NodeType;

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
  Expected {
    expected: NodeType,
  },
}

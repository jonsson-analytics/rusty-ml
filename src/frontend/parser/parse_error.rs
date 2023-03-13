use thiserror::Error;

use super::NodeType;
use crate::frontend::lexemes::Lexeme;
use crate::frontend::tokens::Token;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ParseError
{
  #[error("unexpected end of input")]
  UnexpectedEndOfInput
  {
    expected: Token,
  },
  #[error("unexpected token")]
  UnexpectedToken
  {
    expected: Token,
    actual: Lexeme,
  },
  #[error("expected syntax ...")]
  Expected
  {
    expected: NodeType,
  },
}

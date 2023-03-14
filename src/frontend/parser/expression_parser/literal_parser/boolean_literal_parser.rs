use super::*;

pub trait BooleanLiteralParser
where
  Self: TrueLiteralParser,
  Self: FalseLiteralParser,
{
  fn expect_boolean_literal(&mut self) -> Result<surface::Literal>
  {
    attempt!(self as s => s.expect_boolean_true());
    attempt!(self as s => s.expect_boolean_false());
    Err(ParseError::Expected {
      expected: NodeType::BooleanLiteral,
    })
  }
}

impl<Lexer> BooleanLiteralParser for Lexer
where
  Self: TrueLiteralParser,
  Self: FalseLiteralParser,
{
}

#[cfg(test)]
mod spec
{
  use pretty_assertions::assert_eq;

  use super::*;
  use crate::frontend::lexer::Lexer;

  #[test]
  fn keyword_true()
  {
    let mut lexer = Lexer::from_str("true").with_backtracking();
    assert_eq!(
      lexer.expect_boolean_literal(),
      Ok(surface::Literal::Boolean(true))
    )
  }

  #[test]
  fn keyword_false()
  {
    let mut lexer = Lexer::from_str("false").with_backtracking();
    assert_eq!(
      lexer.expect_boolean_literal(),
      Ok(surface::Literal::Boolean(false))
    )
  }
}

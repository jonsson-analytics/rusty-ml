use super::*;

pub trait NumericLiteralParser
where
  Self: ExpectSyntax,
  Self: CanBacktrack,
{
  fn expect_numeric_literal(&mut self) -> Result<surface::Literal>
  {
    self
      .expect(Token::NumericLiteral)
      .map(|lexeme| surface::Literal::Numeric(lexeme.value().clone()))
  }
}
impl<Lexer> NumericLiteralParser for Lexer
where
  Self: ExpectSyntax,
  Self: CanBacktrack,
{
}

#[cfg(test)]
mod spec
{
  use pretty_assertions::assert_eq;

  use super::*;
  use crate::frontend::lexer::Lexer;

  #[test]
  fn can_parse_numerics()
  {
    let mut lexer = Lexer::from_str("1").with_backtracking();
    assert_eq!(
      lexer.expect_numeric_literal(),
      Ok(surface::Literal::Numeric("1".into()))
    )
  }

  #[test]
  fn produces_useful_error_on_malformed_numerics()
  {
    let mut lexer = Lexer::from_str("1.1.1").with_backtracking();
    assert_eq!(
      lexer.expect_numeric_literal(),
      Err(ParseError::UnexpectedToken {
        expected: Token::NumericLiteral,
        actual: Lexeme::malformed_numeric("1.1.1"),
      })
    )
  }
}

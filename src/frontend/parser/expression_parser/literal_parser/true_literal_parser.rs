use super::*;

pub trait TrueLiteralParser
where
  Self: ExpectSyntax,
  Self: CanBacktrack,
{
  fn expect_boolean_true(&mut self) -> Result<surface::Literal>
  {
    self
      .expect(Token::Keyword("true"))
      .map(|_| surface::Literal::Boolean(true))
  }
}
impl<Lexer> TrueLiteralParser for Lexer
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
  fn keyword_true()
  {
    let mut lexer = Lexer::from_str("true").with_backtracking();
    assert_eq!(lexer.expect_boolean_true(), Ok(surface::Literal::Boolean(true)))
  }
}

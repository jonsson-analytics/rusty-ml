use super::*;

pub trait FalseLiteralParser
where
  Self: ExpectSyntax,
  Self: CanBacktrack,
{
  fn expect_boolean_false(&mut self) -> Result<surface::Literal>
  {
    self
      .expect(Token::Keyword("false"))
      .map(|_| surface::Literal::Boolean(false))
  }
}
impl<Lexer> FalseLiteralParser for Lexer
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
  fn keyword_false()
  {
    let mut lexer = Lexer::from_str("false").with_backtracking();
    assert_eq!(
      lexer.expect_boolean_false(),
      Ok(surface::Literal::Boolean(false))
    )
  }
}

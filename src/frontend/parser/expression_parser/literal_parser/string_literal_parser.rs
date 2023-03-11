use super::*;

pub trait StringLiteralParser
where
  Self: ExpectSyntax,
  Self: CanBacktrack,
{
  fn expect_string_literal(&mut self) -> Result<surface::Literal>
  {
    self
      .expect(Token::StringLiteral)
      .map(|lexeme| surface::Literal::String(lexeme.value().clone()))
  }
}
impl<Lexer> StringLiteralParser for Lexer
where
  Self: ExpectSyntax,
  Self: CanBacktrack,
{
}

#[cfg(test)]
mod spec
{
  use super::*;
  use crate::frontend::lexer::Lexer;

  #[test]
  fn keyword_true()
  {
    let mut lexer = Lexer::from_str("`foo`").with_backtracking();
    assert_eq!(
      lexer.expect_string_literal(),
      Ok(surface::Literal::String("foo".into()))
    )
  }
}

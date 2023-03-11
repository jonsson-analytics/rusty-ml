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

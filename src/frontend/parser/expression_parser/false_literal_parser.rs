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
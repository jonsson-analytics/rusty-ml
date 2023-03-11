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

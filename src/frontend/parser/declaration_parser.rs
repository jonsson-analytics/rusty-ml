use super::*;

pub trait DeclarationParser
where
  Self: Iterator<Item = Lexeme>,
  Self: CanBacktrack,
  Self: ExpressionParser,
{
  fn expect_val_binding(&mut self) -> Result<surface::ValBinding>
  {
    let _ = dbg!(self.expect(Token::Keyword("val")))?;
    let name = dbg!(self.expect_identifier())?;
    let _ = dbg!(self.expect(Token::Keyword("=")))?;
    let value = self.expect_expression()?;
    let _ = self.expect(Token::Keyword(";"))?;
    return Ok(surface::ValBinding {
      name,
      value,
    })
  }

  fn expect_def_binding(&mut self) -> Result<surface::DefBinding>
  {
    let _ = self.expect(Token::Keyword("def"))?;
    let name = self.expect_identifier()?;
    let _ = self.expect(Token::Keyword("="))?;
    let value = self.expect_expression()?;
    let _ = self.expect(Token::Keyword(";"))?;
    return Ok(surface::DefBinding {
      name,
      value,
    })
  }
}
impl<Lexer> DeclarationParser for Lexer
where
  Self: Iterator<Item = Lexeme>,
  Self: CanBacktrack,
  Self: ExpressionParser,
{
}
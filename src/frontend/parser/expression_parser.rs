mod literal_parser;

pub use literal_parser::*;

use super::*;


pub trait ExpressionParser
where
  Self: ExpectSyntax,
  Self: CanBacktrack,
  Self: LiteralParser,
{
  fn expect_expression(&mut self) -> Result<surface::Expression>
  {
    self
      .expect_literal()
      .map(|literal| literal.into())
  }

  fn expect_identifier(&mut self) -> Result<surface::Identifier>
  {
    let name = self.expect(Token::Identifier)?;
    return Ok(surface::Identifier {
      name: name.value().into(),
    })
  }
}

impl<Lexer> ExpressionParser for Lexer
where
  Self: Iterator<Item = Lexeme>,
  Self: CanBacktrack,
{
}

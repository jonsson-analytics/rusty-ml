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
    attempt!(self as s => {
      let literal = s.expect_literal()?;
      Ok(literal.into())
    });
    return Err(ParseError::Expected {
      expected: NodeType::Expression,
    })
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

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

#[cfg(test)]
mod spec
{
  use super::*;
  use crate::frontend::lexer::Lexer;

  #[test]
  fn can_parse_string_literal()
  {
    let mut lexer = Lexer::from_str("`foo`").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(surface::Literal::String("foo".into()).into())
    );
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn can_parse_boolean_literal()
  {
    let mut lexer = Lexer::from_str("true false").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(surface::Literal::Boolean(true).into())
    );
    assert_eq!(
      lexer.expect_expression(),
      Ok(surface::Literal::Boolean(false).into())
    );
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn can_parse_numeric_literal()
  {
    let mut lexer = Lexer::from_str("10").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(surface::Literal::Numeric("10".into()).into())
    );
    assert_eq!(lexer.next(), None);
  }
}

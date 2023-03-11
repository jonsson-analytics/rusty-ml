use super::*;

pub trait ExpressionParser
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

  fn expect_boolean_true(&mut self) -> Result<surface::Literal>
  {
    self
      .expect(Token::Keyword("true"))
      .map(|_| surface::Literal::Boolean(true))
  }

  fn expect_boolean_false(&mut self) -> Result<surface::Literal>
  {
    self
      .expect(Token::Keyword("false"))
      .map(|_| surface::Literal::Boolean(false))
  }

  fn expect_literal(&mut self) -> Result<surface::Literal>
  {
    let string_literal = self.breakpoint(|bp| bp.expect_string_literal());
    let boolean_true = self.breakpoint(|bp| bp.expect_boolean_true());
    let boolean_false = self.breakpoint(|bp| bp.expect_boolean_false());

    return string_literal
      .or(boolean_true)
      .or(boolean_false)
      .map_err(|_| ParseError::Expected {
        expected: NodeType::Literal,
      })
  }

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
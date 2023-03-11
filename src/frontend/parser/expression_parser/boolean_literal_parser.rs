use super::*;

pub trait BooleanLiteralParser
where
  Self: TrueLiteralParser,
  Self: FalseLiteralParser,
{
  fn expect_boolean_literal(&mut self) -> Result<surface::Literal>
  {
    let boolean_true = self.breakpoint(|bp| bp.expect_boolean_true());
    let boolean_false = self.breakpoint(|bp| bp.expect_boolean_false());
    return boolean_true
      .or(boolean_false)
      .map_err(|_| ParseError::Expected {
        expected: NodeType::BooleanLiteral,
      })
  }
}

impl<Lexer> BooleanLiteralParser for Lexer
where
  Self: TrueLiteralParser,
  Self: FalseLiteralParser,
{
}

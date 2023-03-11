use super::*;

pub trait LiteralParser
where
  Self: StringLiteralParser,
  Self: BooleanLiteralParser,
{
  fn expect_literal(&mut self) -> Result<surface::Literal>
  {
    let string_literal = self.breakpoint(|bp| bp.expect_string_literal());
    let boolean_literal = self.expect_boolean_literal();

    return string_literal
      .or(boolean_literal)
      .map_err(|_| ParseError::Expected {
        expected: NodeType::Literal,
      })
  }
}
impl<Lexer> LiteralParser for Lexer
where
  Self: ExpectSyntax,
  Self: CanBacktrack,
{
}

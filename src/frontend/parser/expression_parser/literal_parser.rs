mod boolean_literal_parser;
mod false_literal_parser;
mod numeric_literal_parser;
mod string_literal_parser;
mod true_literal_parser;

pub use boolean_literal_parser::*;
pub use false_literal_parser::*;
pub use numeric_literal_parser::*;
pub use string_literal_parser::*;
pub use true_literal_parser::*;

use super::*;

pub trait LiteralParser
where
  Self: Sized,
  Self: StringLiteralParser,
  Self: BooleanLiteralParser,
{
  fn expect_literal(&mut self) -> Result<surface::Literal>
  {
    attempt!(self as s => s.expect_string_literal());
    attempt!(self as s => s.expect_boolean_literal());
    attempt!(self as s => s.expect_numeric_literal());
    return Err(ParseError::Expected {
      expected: NodeType::Literal,
    })
  }
}
impl<Lexer> LiteralParser for Lexer
where
  Self: Sized,
  Self: ExpectSyntax,
  Self: CanBacktrack,
{
}

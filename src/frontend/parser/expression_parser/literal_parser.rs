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

#[cfg(test)]
mod spec
{
  use pretty_assertions::assert_eq;

  use super::*;
  use crate::frontend::lexer::Lexer;

  #[test]
  fn can_parse_string_literal()
  {
    let mut lexer = Lexer::from_str("`foo`").with_backtracking();
    assert_eq!(
      lexer.expect_literal(),
      Ok(surface::Literal::String("foo".into()))
    );
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn can_parse_boolean_literal()
  {
    let mut lexer = Lexer::from_str("true false").with_backtracking();
    assert_eq!(lexer.expect_literal(), Ok(surface::Literal::Boolean(true)));
    assert_eq!(lexer.expect_literal(), Ok(surface::Literal::Boolean(false)));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn can_parse_numeric_literal()
  {
    let mut lexer = Lexer::from_str("10").with_backtracking();
    assert_eq!(
      lexer.expect_literal(),
      Ok(surface::Literal::Numeric("10".into()))
    );
    assert_eq!(lexer.next(), None);
  }
}

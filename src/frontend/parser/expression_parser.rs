mod abstraction_parser;
mod literal_parser;

pub use abstraction_parser::*;
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
      let abstraction = s.expect_abstraction()?;
      return Ok(abstraction.into())
    });
    attempt!(self as s => {
      let literal = s.expect_literal()?;
      return Ok(literal.into())
    });
    attempt!(self as s => {
      let identifier = s.expect_identifier()?;
      return Ok(identifier.into())
    });
    return Err(ParseError::Expected {
      expected: NodeType::Expression,
    })
  }
}

impl<Lexer> ExpressionParser for Lexer
where
  Self: ExpectSyntax,
  Self: CanBacktrack,
{
}

#[cfg(test)]
mod spec
{
  use super::*;
  use crate::frontend::lexer::Lexer;

  #[test]
  fn can_parse_parentheses()
  {
    let mut lexer = Lexer::from_str("(foo)").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(surface::Identifier::new("foo").into()),
    );
    assert_eq!(lexer.next(), None);

    let mut lexer = Lexer::from_str("((foo))").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(surface::Identifier::new("foo").into()),
    );
    assert_eq!(lexer.next(), None);

    let mut lexer = Lexer::from_str("(((foo)))").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(surface::Identifier::new("foo").into()),
    );
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn can_parse_application()
  {
    // todo: fix expectations

    let mut lexer = Lexer::from_str("f a b c").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(
        surface::Application {
          abstraction: surface::Identifier::new("f").into(),
          arguments: vec![
            surface::Identifier::new("a").into(),
            surface::Identifier::new("b").into(),
            surface::Identifier::new("c").into(),
          ],
        }
        .into()
      ),
    );
    assert_eq!(lexer.next(), None);

    let mut lexer = Lexer::from_str("(fun x -> x) 10").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(
        surface::Application {
          abstraction: surface::Abstraction {
            parameters: vec![surface::Identifier::new("x")],
            body: surface::Identifier::new("x").into(),
          }
          .into(),
          arguments: vec![surface::Literal::Numeric("10".into()).into(),],
        }
        .into()
      ),
    );
    assert_eq!(lexer.next(), None);

    let mut lexer = Lexer::from_str("f (fun x -> x) 10").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(
        surface::Application {
          abstraction: surface::Identifier::new("f").into(),
          arguments: vec![
            surface::Abstraction {
              parameters: vec![surface::Identifier::new("x")],
              body: surface::Identifier::new("x").into(),
            }
            .into(),
            surface::Literal::Numeric("10".into()).into(),
          ],
        }
        .into()
      ),
    );
    assert_eq!(lexer.next(), None);

    let mut lexer = Lexer::from_str("f 10 (fun x -> x)").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(
        surface::Application {
          abstraction: surface::Identifier::new("f").into(),
          arguments: vec![
            surface::Literal::Numeric("10".into()).into(),
            surface::Abstraction {
              parameters: vec![surface::Identifier::new("x")],
              body: surface::Identifier::new("x").into(),
            }
            .into(),
          ],
        }
        .into()
      ),
    );
    assert_eq!(lexer.next(), None);

    let mut lexer = Lexer::from_str("(g (fun x -> x)) 10").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(
        surface::Application {
          abstraction: surface::Application {
            abstraction: surface::Identifier::new("g").into(),
            arguments: vec![surface::Abstraction {
              parameters: vec![surface::Identifier::new("x")],
              body: surface::Identifier::new("x").into(),
            }
            .into()],
          }
          .into(),
          arguments: vec![surface::Literal::Numeric("10".into()).into()],
        }
        .into()
      )
    );
    assert_eq!(lexer.next(), None);

    let mut lexer = Lexer::from_str("f (g (fun x -> x) 10) (fun x -> x) 10")
      .with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(
        surface::Application {
          abstraction: surface::Identifier::new("f").into(),
          arguments: vec![
            surface::Application {
              abstraction: surface::Identifier::new("g").into(),
              arguments: vec![surface::Abstraction {
                parameters: vec![surface::Identifier::new("x")],
                body: surface::Identifier::new("x").into(),
              }
              .into()],
            }
            .into(),
            surface::Abstraction {
              parameters: vec![surface::Identifier::new("x")],
              body: surface::Identifier::new("x").into(),
            }
            .into(),
            surface::Literal::Numeric("10".into()).into(),
          ],
        }
        .into()
      ),
    );
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn can_parse_identifier()
  {
    let mut lexer = Lexer::from_str("foo").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(surface::Identifier::new("foo").into())
    );
    assert_eq!(lexer.next(), None);
  }

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

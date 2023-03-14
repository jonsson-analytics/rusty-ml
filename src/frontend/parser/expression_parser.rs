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
  fn expect_expression_value(&mut self) -> Result<surface::Expression>
  {
    attempt!(self as s => {
      let _ = s.expect(Token::Symbol("("))?;
      let inner_expression = s.expect_expression()?;
      let _ = s.expect(Token::Symbol(")"))?;
      Ok(inner_expression)
    });
    attempt!(self as s => {
      let abstraction = s.expect_abstraction()?;
      Ok(abstraction.into())
    });
    attempt!(self as s => {
      let literal = s.expect_literal()?;
      Ok(literal.into())
    });
    attempt!(self as s => {
      let identifier = s.expect_identifier()?;
      Ok(identifier.into())
    });
    Err(ParseError::Expected {
      expected: NodeType::Expression,
    })
  }

  fn expect_expression(&mut self) -> Result<surface::Expression>
  {
    let expression = self.expect_expression_value()?;
    let mut arguments = vec![];
    while let Ok(argument) = self.expect_expression_value() {
      arguments.push(argument);
    }
    Ok(match arguments.is_empty() {
      | true => expression,
      | _ => surface::Application {
        abstraction: expression,
        arguments,
      }
      .into(),
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
  use pretty_assertions::assert_eq;

  use super::*;
  use crate::frontend::lexer::Lexer;

  #[test]
  fn can_parse_parentheses_one_level()
  {
    let mut lexer = Lexer::from_str("(foo)").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(surface::Identifier::new("foo").into()),
    );
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn can_parse_parentheses_two_levels()
  {
    let mut lexer = Lexer::from_str("((foo))").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(surface::Identifier::new("foo").into()),
    );
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn can_parse_parentheses_three_levels()
  {
    let mut lexer = Lexer::from_str("(((foo)))").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(surface::Identifier::new("foo").into()),
    );
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn can_parse_named_application_three_arguments()
  {
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
  }

  #[test]
  fn can_parse_immediate_application_one_argument()
  {
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
  }

  #[test]
  fn can_parse_named_application_two_arguments_first_is_an_abstraction()
  {
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
  }

  #[test]
  fn can_parse_named_application_two_arguments_second_is_an_abstraction()
  {
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
  }

  #[test]
  fn can_parse_curried_application_inner_has_one_argument()
  {
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
  }

  #[test]
  fn can_parse_curried_application_inner_has_two_arguments()
  {
    let mut lexer =
      Lexer::from_str("(g (fun x -> x) 10) 10").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(
        surface::Application {
          abstraction: surface::Application {
            abstraction: surface::Identifier::new("g").into(),
            arguments: vec![
              surface::Abstraction {
                parameters: vec![surface::Identifier::new("x")],
                body: surface::Identifier::new("x").into(),
              }
              .into(),
              surface::Literal::Numeric("10".into()).into(),
            ],
          }
          .into(),
          arguments: vec![surface::Literal::Numeric("10".into()).into()],
        }
        .into()
      )
    );
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn can_parse_named_application_with_nested_application_as_first_argument()
  {
    let mut lexer = Lexer::from_str("f (g a b c) b c").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(
        surface::Application {
          abstraction: surface::Identifier::new("f").into(),
          arguments: vec![
            surface::Application {
              abstraction: surface::Identifier::new("g").into(),
              arguments: vec![
                surface::Identifier::new("a").into(),
                surface::Identifier::new("b").into(),
                surface::Identifier::new("c").into(),
              ],
            }
            .into(),
            surface::Identifier::new("b").into(),
            surface::Identifier::new("c").into(),
          ],
        }
        .into()
      )
    );
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn can_parse_named_application_with_nested_application_as_second_argument()
  {
    let mut lexer = Lexer::from_str("f a (g a b c) c").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(
        surface::Application {
          abstraction: surface::Identifier::new("f").into(),
          arguments: vec![
            surface::Identifier::new("a").into(),
            surface::Application {
              abstraction: surface::Identifier::new("g").into(),
              arguments: vec![
                surface::Identifier::new("a").into(),
                surface::Identifier::new("b").into(),
                surface::Identifier::new("c").into(),
              ],
            }
            .into(),
            surface::Identifier::new("c").into(),
          ],
        }
        .into()
      )
    );
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn can_parse_named_application_with_nested_application_as_third_argument()
  {
    let mut lexer = Lexer::from_str("f a b (g a b c)").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(
        surface::Application {
          abstraction: surface::Identifier::new("f").into(),
          arguments: vec![
            surface::Identifier::new("a").into(),
            surface::Identifier::new("b").into(),
            surface::Application {
              abstraction: surface::Identifier::new("g").into(),
              arguments: vec![
                surface::Identifier::new("a").into(),
                surface::Identifier::new("b").into(),
                surface::Identifier::new("c").into(),
              ],
            }
            .into(),
          ],
        }
        .into()
      )
    );
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn can_parse_named_application_with_nested_application_three_levels()
  {
    let mut lexer = Lexer::from_str("f (g (h a))").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(
        surface::Application {
          abstraction: surface::Identifier::new("f").into(),
          arguments: vec![surface::Application {
            abstraction: surface::Identifier::new("g").into(),
            arguments: vec![surface::Application {
              abstraction: surface::Identifier::new("h").into(),
              arguments: vec![surface::Identifier::new("a").into(),],
            }
            .into(),],
          }
          .into(),],
        }
        .into()
      )
    );
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn can_parse_named_application_first_argument_is_an_application()
  {
    let mut lexer = Lexer::from_str("f (g (fun x -> x) 10) (fun y -> y) 10")
      .with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(
        surface::Application {
          abstraction: surface::Identifier::new("f").into(),
          arguments: vec![
            surface::Application {
              abstraction: surface::Identifier::new("g").into(),
              arguments: vec![
                surface::Abstraction {
                  parameters: vec![surface::Identifier::new("x")],
                  body: surface::Identifier::new("x").into(),
                }
                .into(),
                surface::Literal::Numeric("10".into()).into(),
              ],
            }
            .into(),
            surface::Abstraction {
              parameters: vec![surface::Identifier::new("y")],
              body: surface::Identifier::new("y").into(),
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
  fn can_parse_boolean_literal_true()
  {
    let mut lexer = Lexer::from_str("true").with_backtracking();
    assert_eq!(
      lexer.expect_expression(),
      Ok(surface::Literal::Boolean(true).into())
    );
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn can_parse_boolean_literal_false()
  {
    let mut lexer = Lexer::from_str("false").with_backtracking();
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

use super::*;

pub trait DeclarationParser
where
  Self: Iterator<Item = Lexeme>,
  Self: CanBacktrack,
  Self: ExpressionParser,
{
  fn expect_val_binding(&mut self) -> Result<surface::ValBinding>
  {
    let _ = dbg!(self.expect(Token::Keyword("val")))?;
    let name = dbg!(self.expect_identifier())?;
    let _ = dbg!(self.expect(Token::Keyword("=")))?;
    let value = self.expect_expression()?;
    let _ = self.expect(Token::Keyword(";"))?;
    return Ok(surface::ValBinding {
      name,
      value,
    })
  }
}
impl<Lexer> DeclarationParser for Lexer
where
  Self: Iterator<Item = Lexeme>,
  Self: CanBacktrack,
  Self: ExpressionParser,
{
}

#[cfg(test)]
mod tests
{
  use super::super::*;
  use crate::frontend::lexer::Lexer;
  use crate::syntax::surface;

  #[test]
  fn val_foo_is_str_bar()
  {
    let mut lexer = Lexer::from_str("val foo = `bar`;").with_backtracking();
    assert_eq!(
      lexer.expect_val_binding(),
      Ok(
        surface::ValBinding {
          name: surface::Identifier {
            name: "foo".to_string()
          },
          value: surface::Literal::String("bar".into()).into(),
        }
        .into()
      )
    );
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn val_foo_is_10()
  {
    let mut lexer = Lexer::from_str("val foo = 10;").with_backtracking();
    assert_eq!(
      lexer.expect_val_binding(),
      Ok(
        surface::ValBinding {
          name: surface::Identifier {
            name: "foo".into(),
          },
          value: surface::Literal::Numeric("10".into()).into(),
        }
        .into()
      )
    );
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn val_foo_is_true()
  {
    let mut lexer = Lexer::from_str("val foo = true ;").with_backtracking();
    assert_eq!(
      lexer.expect_val_binding(),
      Ok(
        surface::ValBinding {
          name: surface::Identifier {
            name: "foo".into()
          },
          value: surface::Literal::Boolean(true).into(),
        }
        .into()
      )
    );
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn val_f_is_fun_x_to_x()
  {
    let mut lexer = Lexer::from_str("val f = fun x -> x ;").with_backtracking();
    assert_eq!(
      lexer.expect_val_binding(),
      Ok(
        surface::ValBinding {
          name: surface::Identifier {
            name: "f".into()
          },
          value: surface::Abstraction {
            parameters: vec![surface::Identifier {
              name: "x".into()
            }],
            body: surface::Identifier {
              name: "x".into()
            }
            .into(),
          }
          .into(),
        }
        .into()
      )
    );
    assert_eq!(lexer.next(), None);
  }
}

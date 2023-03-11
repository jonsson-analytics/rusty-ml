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

  fn expect_def_binding(&mut self) -> Result<surface::DefBinding>
  {
    let _ = self.expect(Token::Keyword("def"))?;
    let name = self.expect_identifier()?;
    let _ = self.expect(Token::Keyword("="))?;
    let value = self.expect_expression()?;
    let _ = self.expect(Token::Keyword(";"))?;
    return Ok(surface::DefBinding {
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
          value: surface::Literal::Number(10.0).into(),
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

  #[test]
  fn def_g_x_to_x()
  {
    let mut lexer = Lexer::from_str("def g = fun x -> x ;").with_backtracking();
    assert_eq!(
      lexer.expect_def_binding(),
      Ok(
        surface::DefBinding {
          name: surface::Identifier {
            name: "g".into()
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
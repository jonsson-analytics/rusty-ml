use super::*;

pub trait DeclarationParser
where
  Self: Iterator<Item = Lexeme>,
  Self: CanBacktrack,
  Self: ExpressionParser,
{
  fn expect_val_binding(&mut self) -> Result<surface::ValBinding>
  {
    let _ = self.expect(Token::Keyword("val"))?;
    let name = self.expect_identifier()?;
    let _ = self.expect(Token::Keyword("="))?;
    let value = self.expect_expression()?;
    let _ = self.expect(Token::Keyword(";"))?;
    Ok(surface::ValBinding {
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
  use pretty_assertions::assert_eq;

  use super::super::*;
  use crate::frontend::lexer::Lexer;
  use crate::syntax::surface;

  #[test]
  fn val_foo_is_str_bar()
  {
    let mut lexer = Lexer::from_str("val foo = `bar` ;").with_backtracking();
    assert_eq!(
      lexer.expect_val_binding(),
      Ok(
        surface::ValBinding {
          name: surface::Identifier::new("foo"),
          value: surface::Literal::String("bar".into()).into(),
        }
      )
    );
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn val_foo_is_10()
  {
    let mut lexer = Lexer::from_str("val foo = 10 ;").with_backtracking();
    assert_eq!(
      lexer.expect_val_binding(),
      Ok(
        surface::ValBinding {
          name: surface::Identifier::new("foo"),
          value: surface::Literal::Numeric("10".into()).into(),
        }
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
          name: surface::Identifier::new("foo"),
          value: surface::Literal::Boolean(true).into(),
        }
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
          name: surface::Identifier::new("f"),
          value: surface::Abstraction {
            parameters: vec![surface::Identifier::new("x")],
            body: surface::Identifier::new("x").into(),
          }
          .into(),
        }
      )
    );
    assert_eq!(lexer.next(), None);
  }
}

#[cfg(test)]
mod top_level
{
  use super::super::*;
  use crate::frontend::syntax::*;
  use crate::frontend::common::*;

  #[test]
  fn val_foo_is_str_bar()
  {
    let mut parser = Parser::from_str("val foo = `bar`;");
    assert_eq!(
      parser.next(),
      Some(Ok(TopLevel::Val(Val {
        name: Identifier {
          name: "foo".to_string()
        },
        value: Expression::Literal(Literal::String("bar".to_string())),
      })))
    );
    assert_eq!(parser.next(), None);
  }

  #[test]
  fn val_foo_is_10()
  {
    let mut parser = Parser::from_str("val foo = 10;");
    assert_eq!(
      parser.next(),
      Some(Ok(TopLevel::Val(Val {
        name: Identifier {
          name: "foo".to_string()
        },
        value: Expression::Literal(Literal::Number(10.0)),
      })))
    );
    assert_eq!(parser.next(), None);
  }

  #[test]
  fn val_foo_is_true()
  {
    let mut parser = Parser::from_str("val foo = true;");
    assert_eq!(
      parser.next(),
      Some(Ok(TopLevel::Val(Val {
        name: Identifier {
          name: "foo".to_string()
        },
        value: Expression::Literal(Literal::Boolean(true)),
      })))
    );
    assert_eq!(parser.next(), None);
  }

  #[test]
  fn val_f_is_fun_x_to_x()
  {
    let mut parser = Parser::from_str("val f = fun x -> x;");
    assert_eq!(
      parser.next(),
      Some(Ok(TopLevel::Val(Val {
        name: Identifier {
          name: "f".to_string()
        },
        value: Expression::Abstraction(Abstraction {
          parameters: vec![Identifier {
            name: "x".to_string()
          }],
          body: Box::new(Expression::Identifier(Identifier {
            name: "x".to_string()
          })),
        }),
      })))
    );
    assert_eq!(parser.next(), None);
  }

  #[test]
  fn def_g_x_to_x()
  {
    let mut parser = Parser::from_str("def g x -> x;");
    assert_eq!(
      parser.next(),
      Some(Ok(TopLevel::Val(Val {
        name: Identifier {
          name: "g".to_string()
        },
        value: Expression::Abstraction(Abstraction {
          parameters: vec![Identifier {
            name: "x".to_string()
          }],
          body: Box::new(Expression::Identifier(Identifier {
            name: "x".to_string()
          })),
        }),
      })))
    );
    assert_eq!(parser.next(), None);
  }
}

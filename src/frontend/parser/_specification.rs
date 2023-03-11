#[cfg(test)]
mod top_level
{
  use super::super::*;
  use crate::syntax::surface;

  #[test]
  fn val_foo_is_str_bar()
  {
    let mut parser = Parser::from_str("val foo = `bar`;");
    assert_eq!(
      parser.next(),
      Some(Ok(
        surface::Val {
          name: surface::Identifier {
            name: "foo".to_string()
          },
          value: surface::Literal::String("bar".into()).into(),
        }
        .into()
      ))
    );
    assert_eq!(parser.next(), None);
  }

  #[test]
  fn val_foo_is_10()
  {
    let mut parser = Parser::from_str("val foo = 10;");
    assert_eq!(
      parser.next(),
      Some(Ok(
        surface::Val {
          name: surface::Identifier {
            name: "foo".into(),
          },
          value: surface::Literal::Number(10.0).into(),
        }
        .into()
      ))
    );
    assert_eq!(parser.next(), None);
  }

  #[test]
  fn val_foo_is_true()
  {
    let mut parser = Parser::from_str("val foo = true;");
    assert_eq!(
      parser.next(),
      Some(Ok(
        surface::Val {
          name: surface::Identifier {
            name: "foo".into()
          },
          value: surface::Literal::Boolean(true).into(),
        }
        .into()
      ))
    );
    assert_eq!(parser.next(), None);
  }

  #[test]
  fn val_f_is_fun_x_to_x()
  {
    let mut parser = Parser::from_str("val f = fun x -> x;");
    assert_eq!(
      parser.next(),
      Some(Ok(
        surface::Val {
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
      ))
    );
    assert_eq!(parser.next(), None);
  }

  #[test]
  fn def_g_x_to_x()
  {
    let mut parser = Parser::from_str("def g x -> x;");
    assert_eq!(
      parser.next(),
      Some(Ok(
        surface::Val {
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
      ))
    );
    assert_eq!(parser.next(), None);
  }
}

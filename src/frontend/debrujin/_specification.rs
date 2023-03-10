#[cfg(test)]
mod expressions
{

  use std::assert_matches::assert_matches;

  use crate::frontend::debrujin::TransformInto;
  use crate::frontend::{
    common,
    debrujin,
    syntax,
  };

  #[test]
  fn literals_remain_the_same()
  {
    let mut environment = vec![];
    let expression = syntax::Expression::literal(true);
    assert_matches!(
      expression.encode(&mut environment),
      Ok(debrujin::Expression::Literal(common::Literal::Boolean(true))),
    );

    let mut environment = vec![];
    let expression = syntax::Expression::literal(false);
    assert_eq!(
      expression.encode(&mut environment),
      Ok(debrujin::Expression::literal(false))
    );

    let mut environment = vec![];
    let expression = syntax::Expression::literal(10.0);
    assert_eq!(
      expression.encode(&mut environment),
      Ok(debrujin::Expression::literal(10.0))
    );

    let mut environment = vec![];
    let expression = syntax::Expression::literal("foo");
    assert_eq!(
      expression.encode(&mut environment),
      Ok(debrujin::Expression::literal("foo"))
    );
  }

  #[test]
  fn free_variable_simple_expression()
  {
    let mut environment = vec![];
    let expression = syntax::Expression::identifier("foo");
    assert_eq!(
      expression.encode(&mut environment),
      Err(debrujin::TransformError::free_variable("foo"))
    );
  }

  #[test]
  fn free_variable_inside_abstraction()
  {
    let mut environment = vec![];
    let expression = syntax::Expression::Abstraction(syntax::Abstraction {
      parameters: vec![
        syntax::Identifier {
          name: "x".to_string(),
        },
        syntax::Identifier {
          name: "y".to_string(),
        },
      ],
      body: Box::new(syntax::Expression::identifier("foo")),
    });
    assert_eq!(
      expression.encode(&mut environment),
      Err(debrujin::TransformError::free_variable("foo"))
    );
  }

  #[test]
  fn no_free_variable_inside_abstraction_first_parameter()
  {
    let mut environment = vec![];
    let expression = syntax::Expression::Abstraction(syntax::Abstraction {
      parameters: vec![
        syntax::Identifier {
          name: "x".to_string(),
        },
        syntax::Identifier {
          name: "y".to_string(),
        },
      ],
      body: Box::new(syntax::Expression::identifier("x")),
    });
    assert_eq!(
      expression.encode(&mut environment),
      Ok(
        debrujin::Abstraction {
          body: Box::new(
            debrujin::Abstraction {
              body: Box::new(debrujin::Expression::identifier(1)),
            }
            .into()
          )
        }
        .into()
      )
    );

    #[test]
    fn no_free_variable_inside_abstraction_second_parameter()
    {
      let mut environment = vec![];
      let expression = syntax::Expression::Abstraction(syntax::Abstraction {
        parameters: vec![
          syntax::Identifier {
            name: "x".to_string(),
          },
          syntax::Identifier {
            name: "y".to_string(),
          },
        ],
        body: Box::new(syntax::Expression::identifier("y")),
      });
      assert_eq!(
        expression.encode(&mut environment),
        Ok(
          debrujin::Abstraction {
            body: Box::new(
              debrujin::Abstraction {
                body: Box::new(debrujin::Expression::identifier(0)),
              }
              .into()
            )
          }
          .into()
        )
      );
    }
  }
}

#[cfg(test)]
mod expressions
{
  use crate::syntax::{
    debrujin,
    surface,
  };
  use crate::transform_into::TransformInto;

  #[test]
  fn literals_remain_the_same()
  {
    let mut environment = vec![];
    let expression: surface::Expression =
      surface::Literal::Boolean(true).into();
    assert_eq!(
      expression.encode(&mut environment),
      Ok(debrujin::Literal::Boolean(true).into()),
    );

    let mut environment = vec![];
    let expression: surface::Expression =
      surface::Literal::Boolean(false).into();
    assert_eq!(
      expression.encode(&mut environment),
      Ok(debrujin::Literal::Boolean(false).into())
    );

    let mut environment = vec![];
    let expression: surface::Expression = surface::Literal::Number(10.0).into();
    assert_eq!(
      expression.encode(&mut environment),
      Ok(debrujin::Literal::Number(10.0).into())
    );

    let mut environment = vec![];
    let expression: surface::Expression =
      surface::Literal::String("foo".into()).into();
    assert_eq!(
      expression.encode(&mut environment),
      Ok(debrujin::Literal::String("foo".into()).into())
    );
  }

  #[test]
  fn free_variable_simple_expression()
  {
    let mut environment = vec![];
    let expression: surface::Expression = surface::Identifier {
      name: "foo".into(),
    }
    .into();
    assert_eq!(
      expression.encode(&mut environment),
      Err(debrujin::TransformError::free_variable("foo"))
    );
  }

  #[test]
  fn free_variable_inside_abstraction()
  {
    let mut environment = vec![];
    let expression: surface::Expression = surface::Abstraction {
      parameters: vec![
        surface::Identifier {
          name: "x".into(),
        },
        surface::Identifier {
          name: "y".into(),
        },
      ],
      body: surface::Identifier {
        name: "foo".into(),
      }
      .into(),
    }
    .into();
    assert_eq!(
      expression.encode(&mut environment),
      Err(debrujin::TransformError::free_variable("foo"))
    );
  }

  #[test]
  fn no_free_variable_inside_abstraction_first_parameter()
  {
    let mut environment = vec![];
    let expression: surface::Expression = surface::Abstraction {
      parameters: vec![
        surface::Identifier {
          name: "x".to_string(),
        },
        surface::Identifier {
          name: "y".to_string(),
        },
      ],
      body: surface::Identifier {
        name: "x".into(),
      }
      .into(),
    }
    .into();
    assert_eq!(
      expression.encode(&mut environment),
      Ok(
        debrujin::Abstraction {
          body: debrujin::Abstraction {
            body: debrujin::Identifier {
              name: 1
            }
            .into(),
          }
          .into()
        }
        .into()
      )
    );
  }

  #[test]
  fn no_free_variable_inside_abstraction_second_parameter()
  {
    let mut environment = vec![];
    let expression: surface::Expression = surface::Abstraction {
      parameters: vec![
        surface::Identifier {
          name: "x".into(),
        },
        surface::Identifier {
          name: "y".into(),
        },
      ],
      body: surface::Identifier {
        name: "y".into(),
      }
      .into(),
    }
    .into();
    assert_eq!(
      expression.encode(&mut environment),
      Ok(
        debrujin::Abstraction {
          body: debrujin::Abstraction {
            body: debrujin::Identifier {
              name: 0
            }
            .into(),
          }
          .into()
        }
        .into()
      )
    );
  }
}

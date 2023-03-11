use crate::syntax::{
  debrujin,
  surface,
};
use crate::transform_into::TransformInto;

#[derive(Debug, Clone, PartialEq)]
pub struct DebrujinEncoding<T>(pub Result<T, TransformError>);


#[derive(Debug, Clone, PartialEq)]
pub enum TransformError
{
  FreeVariable(String),
}

impl TransformError
{
  pub fn free_variable<IntoString>(name: IntoString) -> Self
  where
    IntoString: Into<String>,
  {
    Self::FreeVariable(name.into())
  }
}

trait EnvironmentExt
{
  fn lookup(
    &self,
    name: &str,
  ) -> std::result::Result<usize, TransformError>;

  fn with_bindings<TResult>(
    &mut self,
    bindings: &[surface::Identifier],
    computation: impl FnOnce(&mut Self) -> TResult,
  ) -> TResult;
}

impl EnvironmentExt for Vec<String>
{
  fn with_bindings<TResult>(
    &mut self,
    bindings: &[surface::Identifier],
    computation: impl FnOnce(&mut Self) -> TResult,
  ) -> TResult
  {
    for binding in bindings {
      self.push(binding.name.clone());
    }
    let result = computation(self);
    for _ in bindings {
      self.pop();
    }
    return result
  }

  fn lookup(
    &self,
    name: &str,
  ) -> std::result::Result<usize, TransformError>
  {
    self
      .iter()
      .rev()
      .position(|binding| name == binding)
      .ok_or_else(|| TransformError::free_variable(name.clone()))
  }
}

impl TransformInto<DebrujinEncoding<debrujin::Expression>>
  for surface::Identifier
{
  type Context<'a> = &'a mut Vec<String>;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> DebrujinEncoding<debrujin::Expression>
  {
    DebrujinEncoding(
      context
        .lookup(self.name.as_str())
        .map(|name| {
          debrujin::Identifier {
            name,
          }
          .into()
        }),
    )
  }
}

impl TransformInto<DebrujinEncoding<debrujin::Expression>>
  for surface::Expression
{
  type Context<'a> = &'a mut Vec<String>;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> DebrujinEncoding<debrujin::Expression>
  {
    match self {
      | surface::Expression::Literal(literal) =>
        DebrujinEncoding(Ok(literal.clone().into())),
      | surface::Expression::Identifier(identifier) =>
        identifier.transform(context),
      | surface::Expression::Abstraction(abstraction) =>
        abstraction.transform(context),
      | surface::Expression::Application(application) =>
        application.transform(context),
    }
  }
}

impl TransformInto<DebrujinEncoding<debrujin::Expression>>
  for surface::Application
{
  type Context<'a> = &'a mut Vec<String>;

  fn transform<'a>(
    &self,
    context: Self::Context<'a>,
  ) -> DebrujinEncoding<debrujin::Expression>
  {
    DebrujinEncoding((|| {
      let DebrujinEncoding(abstraction) = self.abstraction.transform(context);
      let mut abstraction = abstraction?;
      for argument in self.arguments.iter() {
        let DebrujinEncoding(argument) = argument.transform(context);
        abstraction = debrujin::Application {
          abstraction,
          argument: argument?,
        }
        .into();
      }
      return Ok(abstraction)
    })())
  }
}

impl TransformInto<DebrujinEncoding<debrujin::Expression>>
  for surface::Abstraction
{
  type Context<'a> = &'a mut Vec<String>;

  fn transform<'a>(
    &self,
    context: Self::Context<'a>,
  ) -> DebrujinEncoding<debrujin::Expression>
  {
    DebrujinEncoding(context.with_bindings(&self.parameters, |context| {
      let DebrujinEncoding(body) = self.body.transform(context);
      let mut body = body?;
      for _ in self.parameters.iter() {
        body = debrujin::Abstraction {
          body,
        }
        .into();
      }
      return Ok(body)
    }))
  }
}

impl TransformInto<DebrujinEncoding<debrujin::TopLevel>> for surface::TopLevel
{
  type Context<'a> = &'a mut Vec<String>;

  fn transform<'a>(
    &self,
    context: Self::Context<'a>,
  ) -> DebrujinEncoding<debrujin::TopLevel>
  {
    todo!()
  }
}

#[cfg(test)]
mod expressions
{
  use super::*;
  use crate::transform_into::TransformInto;

  #[test]
  fn literals_remain_the_same()
  {
    let mut context = vec![];
    let expression: surface::Expression =
      surface::Literal::Boolean(true).into();
    assert_eq!(
      expression.transform(&mut context),
      DebrujinEncoding(Ok(debrujin::Literal::Boolean(true).into()))
    );

    let mut context = vec![];
    let expression: surface::Expression =
      surface::Literal::Boolean(false).into();
    assert_eq!(
      expression.transform(&mut context),
      DebrujinEncoding(Ok(debrujin::Literal::Boolean(false).into()))
    );

    let mut context = vec![];
    let expression: surface::Expression = surface::Literal::Number(10.0).into();
    assert_eq!(
      expression.transform(&mut context),
      DebrujinEncoding(Ok(debrujin::Literal::Number(10.0).into()))
    );

    let mut context = vec![];
    let expression: surface::Expression =
      surface::Literal::String("foo".into()).into();
    assert_eq!(
      expression.transform(&mut context),
      DebrujinEncoding(Ok(debrujin::Literal::String("foo".into()).into()))
    );
  }

  #[test]
  fn free_variable_simple_expression()
  {
    let mut context = vec![];
    let expression: surface::Expression = surface::Identifier {
      name: "foo".into(),
    }
    .into();
    assert_eq!(
      expression.transform(&mut context),
      DebrujinEncoding(Err(TransformError::free_variable("foo")))
    );
  }

  #[test]
  fn free_variable_inside_abstraction()
  {
    let mut context = vec![];
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
      expression.transform(&mut context),
      DebrujinEncoding(Err(TransformError::free_variable("foo")))
    );
  }

  #[test]
  fn no_free_variable_inside_abstraction_first_parameter()
  {
    let mut context = vec![];
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
      expression.transform(&mut context),
      DebrujinEncoding(Ok(
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
      ))
    );
  }

  #[test]
  fn no_free_variable_inside_abstraction_second_parameter()
  {
    let mut context = vec![];
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
      expression.transform(&mut context),
      DebrujinEncoding(Ok(
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
      ))
    );
  }
}

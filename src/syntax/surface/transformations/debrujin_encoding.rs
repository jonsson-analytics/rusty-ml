use thiserror::Error;

use crate::syntax::{
  debrujin,
  surface,
};
use crate::transform_into::TransformInto;

pub struct Context
{
  stack: Vec<String>,
}

impl Context
{
  fn with_bindings<TResult>(
    &mut self,
    bindings: &[surface::Identifier],
    computation: impl FnOnce(&mut Self) -> TResult,
  ) -> TResult
  {
    for binding in bindings {
      self.stack.push(binding.name.clone());
    }
    let result = computation(self);
    for _ in bindings {
      self.stack.pop();
    }
    return result
  }

  fn lookup(
    &self,
    name: &str,
  ) -> std::result::Result<usize, TransformError>
  {
    self
      .stack
      .iter()
      .rev()
      .position(|binding| name == binding)
      .ok_or_else(|| TransformError::free_variable(name.clone()))
  }
}

impl Default for Context
{
  fn default() -> Self
  {
    Self {
      stack: Vec::new(),
    }
  }
}

pub trait DebrujinEncoding<'a, Representation>
{
  fn debrujin_encoding(
    &self,
    context: &'a mut Context,
  ) -> std::result::Result<Representation, TransformError>;
}

impl<'a, SourceRepresentation, TargetRepresentation>
  DebrujinEncoding<'a, TargetRepresentation> for SourceRepresentation
where
  SourceRepresentation: TransformInto<
    Result<TargetRepresentation, TransformError>,
    Context<'a> = &'a mut Context,
  >,
{
  fn debrujin_encoding(
    &self,
    context: &'a mut Context,
  ) -> Result<TargetRepresentation, TransformError>
  {
    self.transform(context)
  }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum TransformError
{
  #[error("free variable")]
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

impl TransformInto<Result<debrujin::Expression, TransformError>>
  for surface::Identifier
{
  type Context<'a> = &'a mut Context;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> Result<debrujin::Expression, TransformError>
  {
    context
      .lookup(self.name.as_str())
      .map(|name| debrujin::Identifier::new(name).into())
  }
}

impl TransformInto<Result<debrujin::Expression, TransformError>>
  for surface::Expression
{
  type Context<'a> = &'a mut Context;

  fn transform(
    &self,
    context: Self::Context<'_>,
  ) -> Result<debrujin::Expression, TransformError>
  {
    match self {
      | surface::Expression::Literal(literal) => Ok(literal.clone().into()),
      | surface::Expression::Identifier(identifier) =>
        identifier.transform(context),
      | surface::Expression::Abstraction(abstraction) =>
        abstraction.transform(context),
      | surface::Expression::Application(application) =>
        application.transform(context),
    }
  }
}

impl TransformInto<Result<debrujin::Expression, TransformError>>
  for surface::Application
{
  type Context<'a> = &'a mut Context;

  fn transform<'a>(
    &self,
    context: Self::Context<'a>,
  ) -> Result<debrujin::Expression, TransformError>
  {
    let mut abstraction = self.abstraction.debrujin_encoding(context)?;
    for argument in self.arguments.iter() {
      abstraction = debrujin::Application {
        abstraction,
        argument: argument.debrujin_encoding(context)?,
      }
      .into();
    }
    return Ok(abstraction)
  }
}

impl TransformInto<Result<debrujin::Expression, TransformError>>
  for surface::Abstraction
{
  type Context<'a> = &'a mut Context;

  fn transform<'a>(
    &self,
    context: Self::Context<'a>,
  ) -> Result<debrujin::Expression, TransformError>
  {
    context.with_bindings(&self.parameters, |context| {
      let mut body = self.body.debrujin_encoding(context)?;
      for _ in self.parameters.iter() {
        body = debrujin::Abstraction {
          body,
        }
        .into();
      }
      return Ok(body)
    })
  }
}

impl TransformInto<Result<debrujin::TopLevel, TransformError>>
  for surface::TopLevel
{
  type Context<'a> = &'a mut Vec<String>;

  fn transform<'a>(
    &self,
    context: Self::Context<'a>,
  ) -> Result<debrujin::TopLevel, TransformError>
  {
    todo!()
  }
}

#[cfg(test)]
mod expressions
{
  use super::*;

  #[test]
  fn literals_remain_the_same()
  {
    let mut context = Context::default();
    let expression: surface::Expression =
      surface::Literal::Boolean(true).into();
    assert_eq!(
      expression.debrujin_encoding(&mut context),
      Ok(debrujin::Literal::Boolean(true).into())
    );

    let mut context = Context::default();
    let expression: surface::Expression =
      surface::Literal::Boolean(false).into();
    assert_eq!(
      expression.debrujin_encoding(&mut context),
      Ok(debrujin::Literal::Boolean(false).into())
    );

    let mut context = Context::default();
    let expression: surface::Expression =
      surface::Literal::Numeric("10.0".into()).into();
    assert_eq!(
      expression.debrujin_encoding(&mut context),
      Ok(debrujin::Literal::Numeric("10.0".into()).into())
    );

    let mut context = Context::default();
    let expression: surface::Expression =
      surface::Literal::String("foo".into()).into();
    assert_eq!(
      expression.debrujin_encoding(&mut context),
      Ok(debrujin::Literal::String("foo".into()).into())
    );
  }

  #[test]
  fn free_variable_simple_expression()
  {
    let mut context = Context::default();
    let expression: surface::Expression =
      surface::Identifier::new("foo").into();
    assert_eq!(
      expression.debrujin_encoding(&mut context),
      Err(TransformError::free_variable("foo"))
    );
  }

  #[test]
  fn free_variable_inside_abstraction()
  {
    let mut context = Context::default();
    let expression: surface::Expression = surface::Abstraction {
      parameters: vec![
        surface::Identifier::new("x"),
        surface::Identifier::new("y"),
      ],
      body: surface::Identifier::new("foo").into(),
    }
    .into();
    assert_eq!(
      expression.debrujin_encoding(&mut context),
      Err(TransformError::free_variable("foo"))
    );
  }

  #[test]
  fn no_free_variable_inside_abstraction_first_parameter()
  {
    let mut context = Context::default();
    let expression: surface::Expression = surface::Abstraction {
      parameters: vec![
        surface::Identifier::new("x"),
        surface::Identifier::new("y"),
      ],
      body: surface::Identifier::new("x").into(),
    }
    .into();
    assert_eq!(
      expression.debrujin_encoding(&mut context),
      Ok(
        debrujin::Abstraction {
          body: debrujin::Abstraction {
            body: debrujin::Identifier::new(1).into(),
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
    let mut context = Context::default();
    let expression: surface::Expression = surface::Abstraction {
      parameters: vec![
        surface::Identifier::new("x"),
        surface::Identifier::new("y"),
      ],
      body: surface::Identifier::new("y").into(),
    }
    .into();
    assert_eq!(
      expression.debrujin_encoding(&mut context),
      Ok(
        debrujin::Abstraction {
          body: debrujin::Abstraction {
            body: debrujin::Identifier::new(0).into(),
          }
          .into()
        }
        .into()
      )
    );
  }
}

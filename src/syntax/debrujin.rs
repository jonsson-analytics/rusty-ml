mod _specification;
mod common;
mod declaration;
mod expression;
mod top_level;
pub mod transformations;

pub use common::*;
pub use declaration::*;
pub use expression::*;
pub use top_level::*;

use super::surface;
use crate::transform_into::TransformInto;

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

impl TransformInto<Result<Expression, TransformError>> for surface::Expression
{
  type Environment<'a> = &'a mut Vec<String>;

  fn encode(
    &self,
    environment: Self::Environment<'_>,
  ) -> Result<Expression, TransformError>
  {
    match self {
      | surface::Expression::Literal(literal) =>
        Ok(Expression::Literal(literal.clone())),

      | surface::Expression::Identifier(surface::Identifier {
        name,
      }) => environment
        .lookup(name.as_str())
        .map(|name| {
          Expression::Identifier(Identifier {
            name,
          })
        }),
      | surface::Expression::Abstraction(abstraction) =>
        abstraction.encode(environment),
      | surface::Expression::Application(application) =>
        application.encode(environment),
    }
  }
}

impl TransformInto<Result<Expression, TransformError>> for surface::Application
{
  type Environment<'a> = &'a mut Vec<String>;

  fn encode<'a>(
    &self,
    environment: Self::Environment<'a>,
  ) -> Result<Expression, TransformError>
  {
    let mut abstraction = self.abstraction.encode(environment)?;
    for argument in self.arguments.iter() {
      abstraction = Application {
        abstraction,
        argument: argument.encode(environment)?,
      }
      .into();
    }
    return Ok(abstraction)
  }
}

impl TransformInto<Result<Expression, TransformError>> for surface::Abstraction
{
  type Environment<'a> = &'a mut Vec<String>;

  fn encode<'a>(
    &self,
    environment: Self::Environment<'a>,
  ) -> Result<Expression, TransformError>
  {
    environment.with_bindings(self.parameters.as_slice(), |environment| {
      let mut body = self.body.encode(environment)?;
      for _ in self.parameters.iter().rev() {
        body = Abstraction {
          body,
        }
        .into();
      }
      return Ok(body)
    })
  }
}

impl TransformInto<Result<TopLevel, TransformError>> for surface::TopLevel
{
  type Environment<'a> = &'a mut Vec<String>;

  fn encode<'a>(
    &self,
    environment: Self::Environment<'a>,
  ) -> Result<TopLevel, TransformError>
  {
    todo!()
  }
}

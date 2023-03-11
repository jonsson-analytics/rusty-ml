mod _specification;

use super::transform_into::TransformInto;
use super::{
  common,
  syntax,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier
{
  pub name: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Abstraction
{
  pub body: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Application
{
  pub abstraction: Box<Expression>,
  pub argument: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression
{
  Literal(common::Literal),
  Identifier(Identifier),
  Abstraction(Abstraction),
  Application(Application),
}

impl Expression
{
  pub fn literal<IntoLiteral>(value: IntoLiteral) -> Self
  where
    IntoLiteral: Into<common::Literal>,
  {
    Self::Literal(value.into())
  }

  pub fn identifier(value: usize) -> Self
  {
    Self::Identifier(Identifier {
      name: value,
    })
  }

  pub fn abstraction(body: Expression) -> Self
  {
    Self::Abstraction(Abstraction {
      body: Box::new(body),
    })
  }

  pub fn application(
    abstraction: Expression,
    argument: Expression,
  ) -> Self
  {
    Self::Application(Application {
      abstraction: Box::new(abstraction),
      argument: Box::new(argument),
    })
  }
}

impl From<Identifier> for Expression
{
  fn from(identifier: Identifier) -> Self
  {
    Self::Identifier(identifier)
  }
}

impl From<Abstraction> for Expression
{
  fn from(abstraction: Abstraction) -> Self
  {
    Self::Abstraction(abstraction)
  }
}

impl From<Application> for Expression
{
  fn from(application: Application) -> Self
  {
    Self::Application(application)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Val
{
  pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevel
{
  Val(Val),
}

impl From<Val> for TopLevel
{
  fn from(val: Val) -> Self
  {
    Self::Val(val)
  }
}

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
    bindings: &[syntax::Identifier],
    computation: impl FnOnce(&mut Self) -> TResult,
  ) -> TResult;
}

impl EnvironmentExt for Vec<String>
{
  fn with_bindings<TResult>(
    &mut self,
    bindings: &[syntax::Identifier],
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

impl TransformInto<Result<Expression, TransformError>> for syntax::Expression
{
  type Environment<'a> = &'a mut Vec<String>;

  fn encode(
    &self,
    environment: Self::Environment<'_>,
  ) -> Result<Expression, TransformError>
  {
    match self {
      | syntax::Expression::Literal(literal) =>
        Ok(Expression::Literal(literal.clone())),

      | syntax::Expression::Identifier(syntax::Identifier {
        name,
      }) => environment
        .lookup(name.as_str())
        .map(Expression::identifier),

      | syntax::Expression::Abstraction(syntax::Abstraction {
        parameters,
        body,
      }) => environment.with_bindings(parameters.as_slice(), |environment| {
        let mut body = body.encode(environment)?;
        for _ in parameters.iter().rev() {
          body = Expression::abstraction(body);
        }
        return Ok(body)
      }),

      | syntax::Expression::Application(syntax::Application {
        abstraction,
        arguments,
      }) => {
        let mut abstraction = abstraction.encode(environment)?;
        for argument in arguments.iter() {
          abstraction = Expression::Application(Application {
            abstraction: Box::new(abstraction),
            argument: Box::new(argument.encode(environment)?),
          });
        }
        return Ok(abstraction)
      },
    }
  }
}

impl TransformInto<Result<TopLevel, TransformError>> for syntax::TopLevel
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

mod abstraction;
mod application;

pub use abstraction::Abstraction;
pub use application::Application;

pub use super::common::Identifier;
pub use crate::syntax::common::Literal;


#[derive(Debug, Clone, PartialEq)]
pub enum Expression
{
  Literal(Literal),
  Identifier(Identifier),
  Abstraction(Box<Abstraction>),
  Application(Box<Application>),
}

impl From<Literal> for Expression
{
  fn from(literal: Literal) -> Self
  {
    Self::Literal(literal)
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
    Self::Abstraction(Box::new(abstraction))
  }
}

impl From<Application> for Expression
{
  fn from(application: Application) -> Self
  {
    Self::Application(Box::new(application))
  }
}

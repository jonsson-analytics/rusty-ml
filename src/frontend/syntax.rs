use super::common::Literal;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier
{
  pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Abstraction
{
  pub parameters: Vec<Identifier>,
  pub body: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Application
{
  pub abstraction: Box<Expression>,
  pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression
{
  Literal(Literal),
  Identifier(Identifier),
  Abstraction(Abstraction),
  Application(Application),
}

impl Expression
{
  pub fn literal<IntoLiteral>(value: IntoLiteral) -> Self
  where
    IntoLiteral: Into<Literal>,
  {
    Self::Literal(value.into())
  }

  pub fn identifier<IntoString>(value: IntoString) -> Self
  where
    IntoString: Into<String>,
  {
    Self::Identifier(Identifier {
      name: value.into(),
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Val
{
  pub name: Identifier,
  pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevel
{
  Val(Val),
}

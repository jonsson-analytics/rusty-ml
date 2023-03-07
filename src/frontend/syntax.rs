#[derive(Debug, Clone, PartialEq)]
pub enum Literal
{
  String(String),
  Number(f64),
  Boolean(bool),
}

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
  pub argument: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression
{
  Literal(Literal),
  Identifier(Identifier),
  Abstraction(Abstraction),
  Application(Application),
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

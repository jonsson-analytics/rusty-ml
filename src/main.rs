#![feature(result_flattening)]
#![feature(iter_collect_into)]

mod frontend;
mod syntax;
mod transform_into;

use frontend::{
  ExpressionParser,
  Lexer,
  ParseError,
  WithBacktracking,
};
use syntax::debrujin::transformations::Evaluate;
use syntax::surface::transformations::debrujin_encoding::DebrujinEncoding;
use thiserror::Error;

mod generic_syntax
{
  enum Expression<Variable, LetBinding, Application, Abstraction>
  {
    Variable(Variable),
    LetBinding(LetBinding),
    Application(Application),
    Abstraction(Abstraction),
  }

  struct SurfaceLetBinding
  {
    pub name: String,
    pub value: Box<SurfaceExpression>,
    pub body: Box<SurfaceExpression>,
  }

  struct SurfaceApplication
  {
    pub abstraction: Box<SurfaceExpression>,
    pub argument: Box<SurfaceExpression>,
  }

  struct SurfaceAbstraction
  {
    pub name: String,
    pub body: Box<SurfaceExpression>,
  }

  type SurfaceExpression = Expression<
    String,
    SurfaceLetBinding,
    SurfaceApplication,
    SurfaceAbstraction,
  >;

  impl SurfaceExpression
  {
    pub fn desugar(&self) -> DesugaredExpression
    {
      match self {
        | Expression::Variable(variable) =>
          Expression::Variable(variable.clone()),
        | Expression::LetBinding(let_binding) =>
          Expression::Application(DesugaredApplication {
            abstraction: Box::new(Expression::Abstraction(
              DesugaredAbstraction {
                name: let_binding.name.clone(),
                body: Box::new(let_binding.body.desugar()),
              },
            )),
            argument: Box::new(let_binding.value.desugar()),
          }),
        | Expression::Application(application) =>
          Expression::Application(DesugaredApplication {
            abstraction: Box::new(application.abstraction.desugar()),
            argument: Box::new(application.abstraction.desugar()),
          }),
        | Expression::Abstraction(abstraction) =>
          Expression::Abstraction(DesugaredAbstraction {
            name: abstraction.name.clone(),
            body: Box::new(abstraction.body.desugar()),
          }),
      }
    }
  }

  struct DesugaredVariable
  {
    pub name: String,
  }

  struct DesugaredApplication
  {
    pub abstraction: Box<DesugaredExpression>,
    pub argument: Box<DesugaredExpression>,
  }

  struct DesugaredAbstraction
  {
    pub name: String,
    pub body: Box<DesugaredExpression>,
  }

  enum Erased {}

  type DesugaredExpression =
    Expression<String, Erased, DesugaredApplication, DesugaredAbstraction>;

  impl DesugaredExpression
  {
    pub fn debrujin_encode(
      &self,
      stack: &mut Vec<String>,
    ) -> DebrujinEncodedExpression
    {
      match self {
        | Expression::Abstraction(abstraction) => {
          stack.push(abstraction.name.clone());
          let body = abstraction.body.debrujin_encode(stack);
          stack.pop();
          Expression::Abstraction(DebrujinEncodedAbstraction {
            body: Box::new(body),
          })
        },
        | Expression::Application(application) =>
          Expression::Application(DebrujinEncodedApplication {
            abstraction: Box::new(application.abstraction.debrujin_encode(stack)),
            argument: Box::new(application.argument.debrujin_encode(stack)),
          }),
        | Expression::Variable(variable) => Expression::Variable(
          stack
            .iter()
            .rev()
            .position(|binding| binding == variable)
            .expect(format!("{variable} is unbound").as_str()),
        ),
        | Expression::LetBinding(_) => unreachable!(),
      }
    }
  }

  struct DebrujinEncodedApplication
  {
    pub abstraction: Box<DebrujinEncodedExpression>,
    pub argument: Box<DebrujinEncodedExpression>,
  }

  struct DebrujinEncodedAbstraction
  {
    pub body: Box<DebrujinEncodedExpression>,
  }

  type DebrujinEncodedExpression = Expression<
    usize,
    Erased,
    DebrujinEncodedApplication,
    DebrujinEncodedAbstraction,
  >;
}

#[derive(Error, Debug)]
enum CompilationError
{
  #[error("failed to parse input")]
  ParseError(#[from] ParseError),
  #[error("failed to debrujin encode")]
  TransformError(
    #[from] syntax::surface::transformations::debrujin_encoding::TransformError,
  ),
}

type Result<T> = std::result::Result<T, CompilationError>;

fn main() -> Result<()>
{
  let program = "(fun x -> x) 10";
  println!("{}", program);
  let mut lexer = Lexer::from_str(program).with_backtracking();
  let s = lexer.expect_expression()?;
  println!("=> {:?}", s);
  let mut context = Default::default();
  let d = s.debrujin_encoding(&mut context)?;
  println!("=> {:?}", d);
  let mut context = Default::default();
  let v = d.evaluate(&mut context);
  println!("=> {:?}", v);
  Ok(())
}

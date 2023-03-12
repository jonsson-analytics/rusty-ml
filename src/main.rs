#![feature(result_flattening)]

mod frontend;
mod syntax;
mod transform_into;

use frontend::{
  ExpressionParser,
  Lexer,
  WithBacktracking,
  ParseError,
};
use syntax::{surface::transformations::debrujin_encoding::DebrujinEncoding, debrujin::transformations::Evaluate};
use thiserror::Error;

#[derive(Error, Debug)]
enum CompilationError
{
  #[error("failed to parse input")]
  ParseError(#[from] ParseError),
  #[error("failed to debrujin encode")]
  TransformError(#[from] syntax::surface::transformations::debrujin_encoding::TransformError),
}

type Result<T> = std::result::Result<T, CompilationError>;

fn main() -> Result<()>
{
  let program = "fun x y -> x";
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

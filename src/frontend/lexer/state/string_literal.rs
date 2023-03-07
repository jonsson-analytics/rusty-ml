use super::State;
use crate::frontend::lexer::feedable::Feedable;
use crate::frontend::lexer::feedable_result::FeedableResult;
use crate::frontend::tokens::Token;

#[derive(Debug, PartialEq)]
pub struct StringLiteral
{
  pub buffer: Vec<char>,
}

impl Feedable for StringLiteral
{
  fn feed(
    &mut self,
    char: Option<char>,
  ) -> FeedableResult
  {
    todo!()
  }
}

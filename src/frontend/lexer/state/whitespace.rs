use super::State;
use crate::frontend::lexer::feedable::Feedable;
use crate::frontend::lexer::feedable_result::FeedableResult;
use crate::frontend::tokens::Token;

pub struct Whitespace;

impl Feedable for Whitespace
{
  fn feed(
    &mut self,
    char: Option<char>,
  ) -> FeedableResult
  {
    todo!()
  }
}

use super::State;
use crate::frontend::lexer::feedable::Feedable;
use crate::frontend::lexer::feedable_result::FeedableResult;

pub struct Whitespace;

impl Feedable for Whitespace
{
  fn feed(
    &mut self,
    char: Option<char>,
  ) -> FeedableResult
  {
    match char {
      | Some(' ' | '\r' | '\n' | '\t') => FeedableResult::Continue,
      | _ => FeedableResult::Transition {
        state: State::empty(),
        consumed: false,
      },
    }
  }
}

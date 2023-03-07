use crate::frontend::lexemes::Lexeme;

use super::state::State;

#[derive(Debug, PartialEq)]
pub enum FeedableResult
{
  Eof,
  Continue,
  Transition
  {
    state: State,
    consumed: bool,
  },
  Finished
  {
    state: State,
    token: Lexeme,
    consumed: bool,
  },
}

impl FeedableResult
{
  pub fn consumed_input(&self) -> bool
  {
    match self {
      | FeedableResult::Eof => true,
      | FeedableResult::Continue => true,
      | FeedableResult::Transition {
        consumed,
        ..
      } => *consumed,
      | FeedableResult::Finished {
        consumed,
        ..
      } => *consumed,
    }
  }
}

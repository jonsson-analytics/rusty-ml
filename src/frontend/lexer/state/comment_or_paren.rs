use super::State;
use crate::frontend::lexer::feedable::Feedable;
use crate::frontend::lexer::feedable_result::FeedableResult;
use crate::frontend::lexemes::Lexeme;

#[derive(Debug, PartialEq)]
pub struct CommentOrParen;

impl Feedable for CommentOrParen
{
  fn feed(
    &mut self,
    char: Option<char>,
  ) -> FeedableResult
  {
    match char {
      | Some('*') => FeedableResult::Transition {
        state: State::comment(),
        consumed: true,
      },
      | _ => FeedableResult::Finished {
        state: State::empty(),
        token: Lexeme::ParenL,
        consumed: false,
      },
    }
  }
}

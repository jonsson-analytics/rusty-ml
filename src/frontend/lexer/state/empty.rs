use super::State;
use crate::frontend::lexer::feedable::Feedable;
use crate::frontend::lexer::feedable_result::FeedableResult;
use crate::frontend::tokens::Token;

pub struct Empty;
impl Feedable for Empty
{
  fn feed(
    &mut self,
    char: Option<char>,
  ) -> FeedableResult
  {
    match char {
      | None => FeedableResult::Eof,
      | Some('(') => FeedableResult::Transition {
        state: State::comment_or_paren(),
        consumed: true,
      },
      | Some(')') => FeedableResult::Finished {
        state: State::empty(),
        token: Token::ParenR,
        consumed: true,
      },
      | Some('{') => FeedableResult::Finished {
        state: State::empty(),
        token: Token::BraceL,
        consumed: true,
      },
      | Some('}') => FeedableResult::Finished {
        state: State::empty(),
        token: Token::BraceR,
        consumed: true,
      },
      | Some('[') => FeedableResult::Finished {
        state: State::empty(),
        token: Token::BracketL,
        consumed: true,
      },
      | Some(']') => FeedableResult::Finished {
        state: State::empty(),
        token: Token::BracketR,
        consumed: true,
      },
      | Some(' ' | '\t' | '\n' | '\r') => FeedableResult::Transition {
        state: State::whitespace(),
        consumed: false,
      },
      | Some(_) => FeedableResult::Transition {
        state: State::identifier(),
        consumed: false,
      },
    }
  }
}

use super::State;
use crate::frontend::lexer::feedable::Feedable;
use crate::frontend::lexer::feedable_result::FeedableResult;
use crate::frontend::lexemes::Lexeme;

#[derive(Debug, PartialEq)]
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
        token: Lexeme::symbol(")"),
        consumed: true,
      },
      | Some('{') => FeedableResult::Finished {
        state: State::empty(),
        token: Lexeme::symbol("{"),
        consumed: true,
      },
      | Some('}') => FeedableResult::Finished {
        state: State::empty(),
        token: Lexeme::symbol("}"),
        consumed: true,
      },
      | Some('[') => FeedableResult::Finished {
        state: State::empty(),
        token: Lexeme::symbol("["),
        consumed: true,
      },
      | Some(']') => FeedableResult::Finished {
        state: State::empty(),
        token: Lexeme::symbol("]"),
        consumed: true,
      },
      | Some(' ' | '\t' | '\n' | '\r') => FeedableResult::Transition {
        state: State::whitespace(),
        consumed: false,
      },
      | Some('`') => FeedableResult::Transition {
        state: State::string_literal(),
        consumed: true,
      },
      | Some(_) => FeedableResult::Transition {
        state: State::identifier(),
        consumed: false,
      },
    }
  }
}

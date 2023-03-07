use super::State;
use crate::frontend::lexer::feedable::Feedable;
use crate::frontend::lexer::feedable_result::FeedableResult;
use crate::frontend::tokens::Token;

#[derive(Debug, PartialEq)]
pub struct Identifier
{
  pub buffer: Vec<char>,
}

fn is_delimiting(char: char) -> bool
{
  match char {
    | ' ' | '\t' | '\r' | '\n' | '(' | ')' | '{' | '}' | '[' | ']' | '`' =>
      true,
    | _ => false,
  }
}

impl Feedable for Identifier
{
  fn feed(
    &mut self,
    char: Option<char>,
  ) -> FeedableResult
  {
    match char {
      | None => FeedableResult::Finished {
        state: State::empty(),
        token: Token::identifier(String::from_iter(self.buffer.iter())),
        consumed: false,
      },
      | Some(c) if is_delimiting(c) => FeedableResult::Finished {
        state: State::empty(),
        token: Token::identifier(String::from_iter(self.buffer.iter())),
        consumed: false,
      },
      | Some(c) => {
        self.buffer.push(c);
        FeedableResult::Continue
      },
    }
  }
}

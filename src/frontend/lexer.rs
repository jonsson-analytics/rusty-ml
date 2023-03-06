mod feedable_result;
mod feedable;
mod state;
mod _specification;

use self::feedable_result::FeedableResult;
use self::state::State;
use super::tokens::Token;

struct Lexer<'a>
{
  source: std::str::Chars<'a>,
  state: State,
  buffer: Option<char>,
}

impl<'a> Lexer<'a>
{
  pub fn from_str(str: &'a str) -> Self
  {
    Self {
      source: str.chars(),
      state: State::empty(),
      buffer: None,
    }
  }

  fn consume_buffer_or_next(&mut self) -> Option<char>
  {
    if self.buffer.is_none() {
      self.source.next()
    }
    else {
      let buffer = self.buffer;
      self.buffer = None;
      buffer
    }
  }
}

impl<'a> Iterator for Lexer<'a>
{
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item>
  {
    let mut current = self.consume_buffer_or_next();
    loop {
      let result = self.state.feed(current);
      let consumed_input = result.consumed_input();
      if !consumed_input {
        self.buffer = current;
      }
      match result {
        | FeedableResult::Eof => return None,
        | FeedableResult::Finished {
          state,
          token,
          ..
        } => {
          self.state = state;
          return Some(token)
        },
        | FeedableResult::Transition {
          state,
          ..
        } => {
          self.state = state;
        },
        | FeedableResult::Continue => { /* do nothing */ },
      }
      if consumed_input {
        current = self.source.next()
      }
    }
  }
}

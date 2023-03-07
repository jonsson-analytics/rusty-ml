mod _specification;
mod feedable;
mod feedable_result;
mod state;

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
    loop {
      let current = dbg!(self.consume_buffer_or_next());
      let result = dbg!(self.state.feed(current));
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
    }
  }
}

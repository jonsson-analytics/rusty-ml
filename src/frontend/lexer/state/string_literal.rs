use super::State;
use crate::frontend::lexer::feedable::Feedable;
use crate::frontend::lexer::feedable_result::FeedableResult;
use crate::frontend::lexemes::Lexeme;

#[derive(Debug, PartialEq)]
pub struct StringLiteral
{
  buffer: Vec<char>,
  escaped: bool,
}

impl Default for StringLiteral
{
  fn default() -> Self
  {
    Self {
      buffer: vec![],
      escaped: false,
    }
  }
}

impl Feedable for StringLiteral
{
  fn feed(
    &mut self,
    char: Option<char>,
  ) -> FeedableResult
  {
    match char {
      | None => FeedableResult::Finished {
        state: State::empty(),
        token: Lexeme::UnclosedString,
        consumed: true,
      },
      | Some('`') if self.escaped => {
        self.buffer.push('`');
        self.escaped = false;
        FeedableResult::Continue
      },
      | Some('`') => FeedableResult::Finished {
        state: State::empty(),
        token: Lexeme::string(String::from_iter(self.buffer.iter())),
        consumed: true,
      },
      | Some('\\') => {
        self.escaped = true;
        FeedableResult::Continue
      },
      | Some(char) => {
        self.buffer.push(char);
        FeedableResult::Continue
      },
    }
  }
}

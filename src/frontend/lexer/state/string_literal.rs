use super::State;
use crate::frontend::lexemes::Lexeme;
use crate::frontend::lexer::feedable::Feedable;
use crate::frontend::lexer::feedable_result::FeedableResult;

#[derive(Debug, PartialEq, Default)]
pub struct StringLiteral
{
  buffer: Vec<char>,
  escaped: bool,
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
        token: Lexeme::unclosed_string(),
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

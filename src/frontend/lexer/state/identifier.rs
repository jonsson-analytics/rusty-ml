use super::State;
use crate::frontend::lexemes::Lexeme;
use crate::frontend::lexer::feedable::Feedable;
use crate::frontend::lexer::feedable_result::FeedableResult;

const RESERVED_WORDS: [&str; 8] =
  ["def", "val", "fun", "->", "=", ";", "true", "false"];

#[derive(Debug, PartialEq)]
pub struct Identifier
{
  pub buffer: Vec<char>,
}

impl Identifier
{
  fn token(&self) -> Lexeme
  {
    let value = String::from_iter(self.buffer.iter());
    RESERVED_WORDS
      .iter()
      .find(|&&word| word == value)
      .map_or_else(|| Lexeme::identifier(value), |&word| Lexeme::keyword(word))
  }
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
        token: self.token(),
        consumed: false,
      },
      | Some(c) if is_delimiting(c) => FeedableResult::Finished {
        state: State::empty(),
        token: self.token(),
        consumed: false,
      },
      | Some(c) => {
        self.buffer.push(c);
        FeedableResult::Continue
      },
    }
  }
}

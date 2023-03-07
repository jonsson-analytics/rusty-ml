use super::State;
use crate::frontend::lexer::feedable::Feedable;
use crate::frontend::lexer::feedable_result::FeedableResult;
use crate::frontend::lexemes::Lexeme;

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
    match value.as_str() {
      | "def" => Lexeme::keyword("def"),
      | "val" => Lexeme::keyword("val"),
      | "fun" => Lexeme::keyword("fun"),
      | "true" => Lexeme::keyword("true"),
      | "false" => Lexeme::keyword("false"),
      | _ => Lexeme::identifier(value),
    }
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

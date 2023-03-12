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
      .map_or_else(
        || match is_numeric(value.as_str()) {
          | IsNumeric::Malformed => Lexeme::malformed_numeric(value),
          | IsNumeric::Yes => Lexeme::numeric(value),
          | IsNumeric::No => Lexeme::identifier(value),
        },
        |&word| Lexeme::keyword(word),
      )
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum IsNumeric
{
  No,
  Yes,
  Malformed,
}

fn is_numeric(input: &str) -> IsNumeric
{
  let mut dots = 0;
  let is_numeric = !input.chars().any(|char| match char {
    | '.' => {
      dots += 1;
      false
    },
    | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | ',' => false,
    | _ => true,
  });
  return match is_numeric {
    | true if dots > 1 => IsNumeric::Malformed,
    | true => IsNumeric::Yes,
    | _ => IsNumeric::No,
  }
}

#[cfg(test)]
mod spec {
    use super::{is_numeric, IsNumeric};

  #[test]
  fn input_is_identifier() {
    assert_eq!(is_numeric("input"), IsNumeric::No);
    assert_eq!(is_numeric("foo10"), IsNumeric::No);
    assert_eq!(is_numeric("10foo"), IsNumeric::No);
  }

  #[test]
  fn input_is_numeric() {
    assert_eq!(is_numeric("0"), IsNumeric::Yes);
    assert_eq!(is_numeric("1"), IsNumeric::Yes);
    assert_eq!(is_numeric("2"), IsNumeric::Yes);
    assert_eq!(is_numeric("3"), IsNumeric::Yes);
    assert_eq!(is_numeric("4"), IsNumeric::Yes);
    assert_eq!(is_numeric("5"), IsNumeric::Yes);
    assert_eq!(is_numeric("6"), IsNumeric::Yes);
    assert_eq!(is_numeric("7"), IsNumeric::Yes);
    assert_eq!(is_numeric("8"), IsNumeric::Yes);
    assert_eq!(is_numeric("9"), IsNumeric::Yes);
    assert_eq!(is_numeric("10"), IsNumeric::Yes);
    assert_eq!(is_numeric("1234567890.0123456789"), IsNumeric::Yes);
    assert_eq!(is_numeric("1,234,567,890.012,345,678,9"), IsNumeric::Yes);
  }

  #[test]
  fn input_is_malformed() {
    assert_eq!(is_numeric("10.0.0"), IsNumeric::Malformed);
    assert_eq!(is_numeric("10.0.0.0"), IsNumeric::Malformed);
    assert_eq!(is_numeric("10.0.0.0.0"), IsNumeric::Malformed);
  }
}

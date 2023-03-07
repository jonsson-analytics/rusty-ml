use super::State;
use crate::frontend::lexer::feedable::Feedable;
use crate::frontend::lexer::feedable_result::FeedableResult;
use crate::frontend::lexemes::Lexeme;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Previous
{
  Irrelevant,
  Star,
  ParenL,
}

impl Previous
{
  fn is_paren_l(self) -> bool
  {
    match self {
      | Previous::ParenL => true,
      | _ => false,
    }
  }

  fn is_star(self) -> bool
  {
    match self {
      | Previous::Star => true,
      | _ => false,
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Comment
{
  level: u64,
  previous: Previous,
}

impl Default for Comment
{
  fn default() -> Self
  {
    Self {
      level: 0,
      previous: Previous::Irrelevant,
    }
  }
}

impl Feedable for Comment
{
  fn feed(
    &mut self,
    char: Option<char>,
  ) -> FeedableResult
  {
    match char {
      | None => FeedableResult::Finished {
        state: State::empty(),
        token: Lexeme::UnclosedComment,
        consumed: true,
      },
      | Some('*') if self.previous.is_paren_l() => {
        self.level += 1;
        self.previous = Previous::Irrelevant;
        FeedableResult::Continue
      },
      | Some('*') => {
        self.previous = Previous::Star;
        FeedableResult::Continue
      },
      | Some(')') if self.previous.is_star() => match self.level {
        | 0 => FeedableResult::Transition {
          state: State::empty(),
          consumed: true,
        },
        | _ => {
          self.level -= 1;
          self.previous = Previous::Irrelevant;
          FeedableResult::Continue
        },
      },
      | Some('(') => {
        self.previous = Previous::ParenL;
        FeedableResult::Continue
      },
      | _ => FeedableResult::Continue,
    }
  }
}

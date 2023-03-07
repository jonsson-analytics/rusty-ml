use super::State;
use crate::frontend::lexer::feedable::Feedable;
use crate::frontend::lexer::feedable_result::FeedableResult;
use crate::frontend::tokens::Token;

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
        token: Token::UnclosedComment,
        consumed: true,
      },
      | Some('*') if self.previous.is_paren_l() => FeedableResult::Transition {
        state: State::Comment(Self {
          level: self.level + 1,
          previous: Previous::Irrelevant,
        }),
        consumed: true,
      },
      | Some('*') => FeedableResult::Transition {
        state: State::Comment(Self {
          level: self.level,
          previous: Previous::Star,
        }),
        consumed: true,
      },
      | Some(')') if self.previous.is_star() => match self.level {
        | 0 => FeedableResult::Transition {
          state: State::empty(),
          consumed: true,
        },
        | _ => FeedableResult::Transition {
          state: State::Comment(Comment {
            level: self.level - 1,
            previous: Previous::Irrelevant,
          }),
          consumed: true,
        },
      },
      Some('(') => FeedableResult::Transition {
        state: State::Comment(Self {
          level: self.level,
          previous: Previous::ParenL,
        }),
        consumed: true,
      },
      | _ => FeedableResult::Continue,
    }
  }
}

use crate::frontend::tokens::Token;

use super::{feedable_result::FeedableResult, feedable::Feedable};

pub enum State
{
  Empty(Empty),
  CommentOrParen(CommentOrParen),
  Identifier(Identifier),
  StringLiteral(StringLiteral),
  Comment(Comment),
  Whitespace(Whitespace),
}

impl State
{
  pub fn feed(
    &mut self,
    char: Option<char>,
  ) -> FeedableResult
  {
    match self {
      | Self::Empty(state) => state.feed(char),
      | Self::CommentOrParen(state) => state.feed(char),
      | Self::Comment(state) => state.feed(char),
      | Self::Identifier(state) => state.feed(char),
      | Self::StringLiteral(state) => state.feed(char),
      | Self::Whitespace(state) => state.feed(char),
    }
  }

  pub fn empty() -> Self
  {
    Self::Empty(Empty)
  }

  pub fn comment_or_paren() -> Self
  {
    Self::CommentOrParen(CommentOrParen)
  }

  pub fn identifier() -> Self
  {
    Self::Identifier(Identifier {
      buffer: vec![],
    })
  }

  pub fn string_literal() -> Self
  {
    Self::StringLiteral(StringLiteral {
      buffer: vec![],
    })
  }

  pub fn comment() -> Self
  {
    Self::Comment(Comment {
      level: 1,
    })
  }

  pub fn whitespace() -> Self
  {
    Self::Whitespace(Whitespace)
  }
}


struct Empty;
impl Feedable for Empty
{
  fn feed(
    &mut self,
    char: Option<char>,
  ) -> FeedableResult
  {
    match char {
      | None => FeedableResult::Eof,
      | Some('(') => FeedableResult::Transition {
        state: State::comment_or_paren(),
        consumed: true,
      },
      | Some(')') => FeedableResult::Finished {
        state: State::empty(),
        token: Token::ParenR,
        consumed: true,
      },
      | Some('{') => FeedableResult::Finished {
        state: State::empty(),
        token: Token::BraceL,
        consumed: true,
      },
      | Some('}') => FeedableResult::Finished {
        state: State::empty(),
        token: Token::BraceR,
        consumed: true,
      },
      | Some('[') => FeedableResult::Finished {
        state: State::empty(),
        token: Token::BracketL,
        consumed: true,
      },
      | Some(']') => FeedableResult::Finished {
        state: State::empty(),
        token: Token::BracketR,
        consumed: true,
      },
      | Some(' ' | '\t' | '\n' | '\r') => FeedableResult::Transition {
        state: State::whitespace(),
        consumed: false,
      },
      | Some(_) => FeedableResult::Transition {
        state: State::identifier(),
        consumed: false,
      },
    }
  }
}

struct CommentOrParen;
impl Feedable for CommentOrParen
{
  fn feed(
    &mut self,
    char: Option<char>,
  ) -> FeedableResult
  {
    match char {
      | Some('*') => FeedableResult::Transition {
        state: State::comment(),
        consumed: true,
      },
      | _ => FeedableResult::Finished {
        state: State::empty(),
        token: Token::ParenL,
        consumed: false,
      },
    }
  }
}

struct Identifier
{
  buffer: Vec<char>,
}
impl Feedable for Identifier
{
  fn feed(
    &mut self,
    char: Option<char>,
  ) -> FeedableResult
  {
    todo!()
  }
}

struct StringLiteral
{
  buffer: Vec<char>,
}
impl Feedable for StringLiteral
{
  fn feed(
    &mut self,
    char: Option<char>,
  ) -> FeedableResult
  {
    todo!()
  }
}

struct Comment
{
  level: u64,
}
impl Feedable for Comment
{
  fn feed(
    &mut self,
    char: Option<char>,
  ) -> FeedableResult
  {
    todo!()
  }
}

struct Whitespace;
impl Feedable for Whitespace
{
  fn feed(
    &mut self,
    char: Option<char>,
  ) -> FeedableResult
  {
    todo!()
  }
}

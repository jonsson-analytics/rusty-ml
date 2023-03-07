mod comment;
mod comment_or_paren;
mod empty;
mod identifier;
mod string_literal;
mod whitespace;

use self::comment::Comment;
use self::comment_or_paren::CommentOrParen;
use self::empty::Empty;
use self::identifier::Identifier;
use self::string_literal::StringLiteral;
use self::whitespace::Whitespace;
use super::feedable::Feedable;
use super::feedable_result::FeedableResult;

#[derive(Debug, PartialEq)]
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
    Self::Comment(Comment::default())
  }

  pub fn whitespace() -> Self
  {
    Self::Whitespace(Whitespace)
  }
}

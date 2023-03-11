use super::{
  CanBacktrack,
  Result,
};
use crate::frontend::lexemes::Lexeme;

pub struct BacktrackingIterator<Lexer>
where
  Lexer: Iterator<Item = Lexeme>,
{
  lexer: Lexer,
  buffer: Vec<Lexeme>,
  cursor: usize,
}

impl<Lexer> BacktrackingIterator<Lexer>
where
  Lexer: Iterator<Item = Lexeme>,
{
  pub fn new(lexer: Lexer) -> Self
  {
    Self {
      lexer,
      buffer: Vec::new(),
      cursor: 0,
    }
  }
}

impl<Lexer> Iterator for BacktrackingIterator<Lexer>
where
  Lexer: Iterator<Item = Lexeme>,
{
  type Item = Lexeme;

  fn next(&mut self) -> Option<Self::Item>
  {
    if self.cursor >= self.buffer.len() {
      let lexeme = self.lexer.next()?;
      self.buffer.push(lexeme);
    }
    let next = self.buffer.get(self.cursor).cloned()?;
    self.cursor += 1;
    return Some(next)
  }
}

impl<Lexer> CanBacktrack for BacktrackingIterator<Lexer>
where
  Lexer: Iterator<Item = Lexeme>,
{
  fn breakpoint<T>(
    &mut self,
    computation: impl FnOnce(&mut Self) -> Result<T>,
  ) -> Result<T>
  {
    let cursor = self.cursor;
    let result = computation(self);
    if let Err(_) = result {
      self.cursor = cursor;
    }
    return result
  }
}


#[cfg(test)]
mod spec
{
  use super::*;
  use crate::frontend::lexer::Lexer;
  use crate::frontend::parser::{
    ExpectSyntax,
    ParseError,
    WithBacktracking,
  };
  use crate::frontend::tokens::Token;

  #[test]
  fn success_in_guarded_parse_consumes_tokens()
  {
    let mut lexer = Lexer::from_str("val foo = ;").with_backtracking();
    assert_eq!(
      lexer.breakpoint(|lexer| {
        lexer.expect(Token::Keyword("val"))?;
        lexer.expect(Token::Identifier)?;
        lexer.expect(Token::Keyword("="))?;
        Ok(())
      }),
      Ok(())
    );
    assert_eq!(lexer.expect(Token::Keyword(";")), Ok(Lexeme::keyword(";")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn failure_in_guarded_parse_does_not_consume_tokens()
  {
    let mut lexer = Lexer::from_str("val foo bar ;").with_backtracking();
    let result = lexer.breakpoint(|lexer| {
      lexer.expect(Token::Keyword("val"))?;
      lexer.expect(Token::Identifier)?;
      lexer.expect(Token::Keyword("="))?;
      Ok(())
    });
    dbg!(lexer.cursor);
    assert_eq!(
      result,
      Err(ParseError::UnexpectedToken {
        expected: Token::Keyword("="),
        actual: Lexeme::identifier("bar"),
      })
    );
    assert_eq!(lexer.expect(Token::Keyword("val")), Ok(Lexeme::keyword("val")));
    assert_eq!(lexer.expect(Token::Identifier), Ok(Lexeme::identifier("foo")));
    assert_eq!(lexer.expect(Token::Identifier), Ok(Lexeme::identifier("bar")));
    assert_eq!(lexer.expect(Token::Keyword(";")), Ok(Lexeme::keyword(";")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn success_in_nested_guarded_parse_consumes_tokens()
  {
    let mut lexer = Lexer::from_str("val foo = ;").with_backtracking();
    assert_eq!(
      lexer.breakpoint(|lexer| {
        lexer.expect(Token::Keyword("val"))?;
        lexer.breakpoint(|lexer| {
          lexer.expect(Token::Identifier)?;
          Ok(())
        })?;
        lexer.expect(Token::Keyword("="))?;
        Ok(())
      }),
      Ok(())
    );
    assert_eq!(lexer.expect(Token::Keyword(";")), Ok(Lexeme::keyword(";")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn failure_in_nested_guarded_parse_does_not_consume_tokens()
  {
    let mut lexer = Lexer::from_str("val foo = ;").with_backtracking();
    assert_eq!(
      lexer.breakpoint(|lexer| {
        lexer.expect(Token::Keyword("val"))?;
        assert_eq!(
          lexer.breakpoint(|lexer| {
            lexer.expect(Token::Keyword("val"))?;
            Ok(())
          }),
          Err(ParseError::UnexpectedToken {
            expected: Token::Keyword("val"),
            actual: Lexeme::identifier("foo")
          })
        );
        lexer.expect(Token::Identifier)?;
        lexer.expect(Token::Keyword("="))?;
        Ok(())
      }),
      Ok(())
    );
    assert_eq!(lexer.expect(Token::Keyword(";")), Ok(Lexeme::keyword(";")));
    assert_eq!(lexer.next(), None);
  }
}

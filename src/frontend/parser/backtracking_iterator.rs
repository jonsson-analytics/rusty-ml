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

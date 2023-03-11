use crate::frontend::lexemes::Lexeme;

use super::BacktrackingIterator;

pub trait WithBacktracking
where
  Self: Iterator<Item = Lexeme>,
  Self: Sized,
{
  fn with_backtracking(self) -> BacktrackingIterator<Self>;
}

impl<Lexer> WithBacktracking for Lexer
where
  Lexer: Iterator<Item = Lexeme>,
{
  fn with_backtracking(self) -> BacktrackingIterator<Self>
  {
    BacktrackingIterator::new(self)
  }
}

use super::tokens::Token;

mod _specification;

struct Lexer
{
}

impl Lexer
{
  pub fn from_str(str: &str) -> Self
  {
    todo!()
  }
}

impl Iterator for Lexer
{
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item>
  {
    todo!()
  }
}

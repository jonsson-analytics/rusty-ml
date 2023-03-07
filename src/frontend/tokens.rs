#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Symbol(String),
  Identifier,
  StringLiteral,
  Keyword(String),
}

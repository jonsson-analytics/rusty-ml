mod _specification;
mod node_type;
mod parse_error;

pub use self::node_type::NodeType;
pub use self::parse_error::ParseError;
use super::lexemes::Lexeme;
use super::tokens::Token;
use crate::syntax::surface;

pub type Result<T> = std::result::Result<T, ParseError>;

pub struct BacktrackingIterator<Lexer>
where
  Lexer: Iterator<Item = Lexeme>,
{
  lexer: Lexer,
  buffer: Vec<Lexeme>,
  cursor: usize,
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
    todo!()
  }
}

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
    BacktrackingIterator {
      lexer: self,
      buffer: Vec::new(),
      cursor: 0,
    }
  }
}

pub trait CanBacktrack
{
  fn breakpoint<T>(
    &mut self,
    computation: impl FnOnce(&mut Self) -> Result<T>,
  ) -> Result<T>;
}

pub trait ExpectSyntax
where
  Self: Iterator<Item = Lexeme>,
{
  fn expect(
    &mut self,
    expected: Token,
  ) -> Result<Lexeme>
  {
    self
      .next()
      .ok_or_else(|| ParseError::UnexpectedEndOfInput {
        expected,
      })
      .map(|lexeme| match () {
        | _ if lexeme.token() == &expected => Ok(lexeme),
        | _ => Err(ParseError::UnexpectedToken {
          expected,
          actual: lexeme,
        }),
      })
      .flatten()
  }
}

impl<Lexer> ExpectSyntax for Lexer where Lexer: Iterator<Item = Lexeme>
{
}

pub trait Parser
where
  Self: ExpectSyntax,
  Self: CanBacktrack,
{
  fn expect_string_literal(&mut self) -> Result<surface::Literal>
  {
    self
      .expect(Token::StringLiteral)
      .map(|lexeme| surface::Literal::String(lexeme.value().clone()))
  }

  fn expect_boolean_true(&mut self) -> Result<surface::Literal>
  {
    self
      .expect(Token::Keyword("true"))
      .map(|_| surface::Literal::Boolean(true))
  }

  fn expect_boolean_false(&mut self) -> Result<surface::Literal>
  {
    self
      .expect(Token::Keyword("false"))
      .map(|_| surface::Literal::Boolean(false))
  }

  fn expect_literal(&mut self) -> Result<surface::Literal>
  {
    let string_literal = self.breakpoint(|bp| bp.expect_string_literal());
    let boolean_true = self.breakpoint(|bp| bp.expect_boolean_true());
    let boolean_false = self.breakpoint(|bp| bp.expect_boolean_false());

    return string_literal
      .or(boolean_true)
      .or(boolean_false)
      .map_err(|_| ParseError::Expected {
        expected: NodeType::Literal,
      })
  }

  fn expect_expression(&mut self) -> Result<surface::Expression>
  {
    self
      .expect_literal()
      .map(|literal| literal.into())
  }

  fn expect_identifier(&mut self) -> Result<surface::Identifier>
  {
    let name = self.expect(Token::Identifier)?;
    return Ok(surface::Identifier {
      name: name.value().into(),
    })
  }

  fn expect_val_binding(&mut self) -> Result<surface::ValBinding>
  {
    let _ = dbg!(self.expect(Token::Keyword("val")))?;
    let name = dbg!(self.expect_identifier())?;
    let _ = dbg!(self.expect(Token::Symbol("=")))?;
    let value = self.expect_expression()?;
    let _ = self.expect(Token::Symbol(";"))?;
    return Ok(surface::ValBinding {
      name,
      value,
    })
  }

  fn expect_def_binding(&mut self) -> Result<surface::DefBinding>
  {
    let _ = self.expect(Token::Keyword("def"))?;
    let name = self.expect_identifier()?;
    let _ = self.expect(Token::Symbol("="))?;
    let value = self.expect_expression()?;
    let _ = self.expect(Token::Symbol(";"))?;
    return Ok(surface::DefBinding {
      name,
      value,
    })
  }
}

impl<Lexer> Parser for Lexer
where
  Lexer: Iterator<Item = Lexeme>,
  Lexer: CanBacktrack,
{
}

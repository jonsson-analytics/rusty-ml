use super::*;

pub trait AbstractionParser
where
  Self: ExpectSyntax,
  Self: CanBacktrack,
  Self: LiteralParser,
{
  fn expect_abstraction(&mut self) -> Result<surface::Abstraction>
  {
    let _ = self.expect(Token::Keyword("fun"))?;
    let parameter = self.expect_identifier()?;
    let mut parameters = vec![parameter];
    while let Ok(parameter) = self.breakpoint(|s| s.expect_identifier()) {
      parameters.push(parameter);
    }
    let _ = self.expect(Token::Keyword("->"))?;
    let body = self.expect_expression()?;
    return Ok(surface::Abstraction {
      parameters,
      body,
    })
  }
}

impl<Lexer> AbstractionParser for Lexer
where
  Self: ExpectSyntax,
  Self: CanBacktrack,
{
}

#[cfg(test)]
mod spec
{
  use pretty_assertions::assert_eq;

  use super::*;
  use crate::frontend::lexer::Lexer;

  #[test]
  fn can_parse_abstraction()
  {
    let mut lexer = Lexer::from_str("fun x -> x").with_backtracking();
    assert_eq!(
      lexer.expect_abstraction(),
      Ok(surface::Abstraction {
        parameters: vec![surface::Identifier::new("x")],
        body: surface::Identifier::new("x").into(),
      })
    );
    assert_eq!(lexer.next(), None);

    let mut lexer = Lexer::from_str("fun x y -> x").with_backtracking();
    assert_eq!(
      lexer.expect_abstraction(),
      Ok(surface::Abstraction {
        parameters: vec![
          surface::Identifier::new("x"),
          surface::Identifier::new("y"),
        ],
        body: surface::Identifier::new("x").into(),
      })
    );
    assert_eq!(lexer.next(), None);

    let mut lexer = Lexer::from_str("fun x y z -> x").with_backtracking();
    assert_eq!(
      lexer.expect_abstraction(),
      Ok(surface::Abstraction {
        parameters: vec![
          surface::Identifier::new("x"),
          surface::Identifier::new("y"),
          surface::Identifier::new("z"),
        ],
        body: surface::Identifier::new("x").into(),
      })
    );
    assert_eq!(lexer.next(), None);
  }
}

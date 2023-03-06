#[cfg(test)]
mod parentheses
{
  use super::super::*;

  #[test]
  fn single_paren_l_then_eof()
  {
    let mut lexer = Lexer::from_str("(");
    assert_eq!(lexer.next(), Some(Token::ParenL));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn multiple_paren_l_then_eof()
  {
    let mut lexer = Lexer::from_str("(((");
    assert_eq!(lexer.next(), Some(Token::ParenL));
    assert_eq!(lexer.next(), Some(Token::ParenL));
    assert_eq!(lexer.next(), Some(Token::ParenL));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn single_paren_l_then_whitespace()
  {
    let mut lexer = Lexer::from_str("( ");
    assert_eq!(lexer.next(), Some(Token::ParenL));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn multiple_paren_l_then_whitespace()
  {
    let mut lexer = Lexer::from_str("((( ");
    assert_eq!(lexer.next(), Some(Token::ParenL));
    assert_eq!(lexer.next(), Some(Token::ParenL));
    assert_eq!(lexer.next(), Some(Token::ParenL));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn whitespace_then_paren_l_then_whitespace()
  {
    let mut lexer = Lexer::from_str(" \t\r\n( \t\r\n");
    assert_eq!(lexer.next(), Some(Token::ParenL));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn paren_r()
  {
    let mut lexer = Lexer::from_str(")");
    assert_eq!(lexer.next(), Some(Token::ParenR));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn whitespace_then_paren_r_then_whitespace()
  {
    let mut lexer = Lexer::from_str(" \t\r\n) \t\r\n");
    assert_eq!(lexer.next(), Some(Token::ParenR));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn brace_l()
  {
    let mut lexer = Lexer::from_str("{");
    assert_eq!(lexer.next(), Some(Token::BraceL));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn whitespace_then_brace_l_then_whitespace()
  {
    let mut lexer = Lexer::from_str(" \t\r\n{ \t\r\n");
    assert_eq!(lexer.next(), Some(Token::BraceL));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn brace_r()
  {
    let mut lexer = Lexer::from_str("}");
    assert_eq!(lexer.next(), Some(Token::BraceR));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn whitespace_then_brace_r_then_whitespace()
  {
    let mut lexer = Lexer::from_str(" \t\r\n} \t\r\n");
    assert_eq!(lexer.next(), Some(Token::BraceR));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn bracket_l()
  {
    let mut lexer = Lexer::from_str("[");
    assert_eq!(lexer.next(), Some(Token::BracketL));
    assert_eq!(lexer.next(), None)
  }

  #[test]
  fn whitespace_then_bracket_l_then_whitespace()
  {
    let mut lexer = Lexer::from_str(" \t\r\n[ \t\r\n");
    assert_eq!(lexer.next(), Some(Token::BracketL));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn bracket_r()
  {
    let mut lexer = Lexer::from_str("]");
    assert_eq!(lexer.next(), Some(Token::BracketR));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn whitespace_then_bracket_r_then_whitespace()
  {
    let mut lexer = Lexer::from_str(" \t\r\n] \t\r\n");
    assert_eq!(lexer.next(), Some(Token::BracketL));
    assert_eq!(lexer.next(), None);
  }
}

#[cfg(test)]
mod comment
{
  use super::super::*;

  #[test]
  fn unclosed()
  {
    let mut lexer = Lexer::from_str("(*");
    assert_eq!(lexer.next(), Some(Token::UnclosedComment));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn unclosed_single_star()
  {
    let mut lexer = Lexer::from_str("(*)");
    assert_eq!(lexer.next(), Some(Token::UnclosedComment));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn closed()
  {
    let mut lexer = Lexer::from_str("(**)");
    assert_eq!(lexer.next(), Some(Token::comment("".to_string())));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn closed_multiline()
  {
    let mut lexer = Lexer::from_str("(*\n\n\n*)");
    assert_eq!(lexer.next(), Some(Token::comment("\n\n\n".to_string())));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn leveled_balanced()
  {
    let mut lexer = Lexer::from_str("(*(**)*)");
    assert_eq!(lexer.next(), Some(Token::comment("(**)".to_string())));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn leveled_imbalanced()
  {
    let mut lexer = Lexer::from_str("(*(**)");
    assert_eq!(lexer.next(), Some(Token::UnclosedComment));
    assert_eq!(lexer.next(), None);
  }
}

#[cfg(test)]
mod string
{
  use super::super::*;

  #[test]
  fn unclosed()
  {
    let mut lexer = Lexer::from_str("`foo");
    assert_eq!(lexer.next(), Some(Token::UnclosedString));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn closed()
  {
    let mut lexer = Lexer::from_str("`foo`");
    assert_eq!(lexer.next(), Some(Token::string("foo".to_string())));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn closed_multiline()
  {
    let mut lexer = Lexer::from_str("`foo\nbar`");
    assert_eq!(lexer.next(), Some(Token::string("foo\nbar".to_string())));
    assert_eq!(lexer.next(), None);
  }
}

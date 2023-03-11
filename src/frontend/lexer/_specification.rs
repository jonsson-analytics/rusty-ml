#[cfg(test)]
mod parentheses
{
  use super::super::*;

  #[test]
  fn single_paren_l_then_eof()
  {
    let mut lexer = Lexer::from_str("(");
    assert_eq!(lexer.next(), Some(Lexeme::symbol("(")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn multiple_paren_l_then_eof()
  {
    let mut lexer = Lexer::from_str("(((");
    assert_eq!(lexer.next(), Some(Lexeme::symbol("(")));
    assert_eq!(lexer.next(), Some(Lexeme::symbol("(")));
    assert_eq!(lexer.next(), Some(Lexeme::symbol("(")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn single_paren_l_then_whitespace()
  {
    let mut lexer = Lexer::from_str("( ");
    assert_eq!(lexer.next(), Some(Lexeme::symbol("(")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn multiple_paren_l_then_whitespace()
  {
    let mut lexer = Lexer::from_str("((( ");
    assert_eq!(lexer.next(), Some(Lexeme::symbol("(")));
    assert_eq!(lexer.next(), Some(Lexeme::symbol("(")));
    assert_eq!(lexer.next(), Some(Lexeme::symbol("(")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn whitespace_then_paren_l_then_whitespace()
  {
    let mut lexer = Lexer::from_str(" \t\r\n( \t\r\n");
    assert_eq!(lexer.next(), Some(Lexeme::symbol("(")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn paren_r()
  {
    let mut lexer = Lexer::from_str(")");
    assert_eq!(lexer.next(), Some(Lexeme::symbol(")")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn whitespace_then_paren_r_then_whitespace()
  {
    let mut lexer = Lexer::from_str(" \t\r\n) \t\r\n");
    assert_eq!(lexer.next(), Some(Lexeme::symbol(")")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn brace_l()
  {
    let mut lexer = Lexer::from_str("{");
    assert_eq!(lexer.next(), Some(Lexeme::symbol("{")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn whitespace_then_brace_l_then_whitespace()
  {
    let mut lexer = Lexer::from_str(" \t\r\n{ \t\r\n");
    assert_eq!(lexer.next(), Some(Lexeme::symbol("{")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn brace_r()
  {
    let mut lexer = Lexer::from_str("}");
    assert_eq!(lexer.next(), Some(Lexeme::symbol("}")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn whitespace_then_brace_r_then_whitespace()
  {
    let mut lexer = Lexer::from_str(" \t\r\n} \t\r\n");
    assert_eq!(lexer.next(), Some(Lexeme::symbol("}")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn bracket_l()
  {
    let mut lexer = Lexer::from_str("[");
    assert_eq!(lexer.next(), Some(Lexeme::symbol("[")));
    assert_eq!(lexer.next(), None)
  }

  #[test]
  fn whitespace_then_bracket_l_then_whitespace()
  {
    let mut lexer = Lexer::from_str(" \t\r\n[ \t\r\n");
    assert_eq!(lexer.next(), Some(Lexeme::symbol("[")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn bracket_r()
  {
    let mut lexer = Lexer::from_str("]");
    assert_eq!(lexer.next(), Some(Lexeme::symbol("]")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn whitespace_then_bracket_r_then_whitespace()
  {
    let mut lexer = Lexer::from_str(" \t\r\n] \t\r\n");
    assert_eq!(lexer.next(), Some(Lexeme::symbol("]")));
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
    assert_eq!(lexer.next(), Some(Lexeme::unclosed_comment()));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn unclosed_single_star()
  {
    let mut lexer = Lexer::from_str("(*)");
    assert_eq!(lexer.next(), Some(Lexeme::unclosed_comment()));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn closed()
  {
    let mut lexer = Lexer::from_str("(**)");
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn closed_multiline()
  {
    let mut lexer = Lexer::from_str("(*\n\n\n*)");
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn leveled_balanced()
  {
    let mut lexer = Lexer::from_str("(*(**)*)");
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn leveled_imbalanced()
  {
    let mut lexer = Lexer::from_str("(*(**)");
    assert_eq!(lexer.next(), Some(Lexeme::unclosed_comment()));
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
    assert_eq!(lexer.next(), Some(Lexeme::unclosed_string()));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn closed()
  {
    let mut lexer = Lexer::from_str("`foo`");
    assert_eq!(lexer.next(), Some(Lexeme::string("foo")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn closed_multiline()
  {
    let mut lexer = Lexer::from_str("`foo\nbar`");
    assert_eq!(lexer.next(), Some(Lexeme::string("foo\nbar")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn escaped()
  {
    let mut lexer = Lexer::from_str("`foo\\`bar`");
    assert_eq!(lexer.next(), Some(Lexeme::string("foo`bar")));
    assert_eq!(lexer.next(), None);
  }
}

#[cfg(test)]
mod identifier
{
  use super::super::*;

  #[test]
  fn identifier_then_eof()
  {
    let mut lexer = Lexer::from_str("foo");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("foo")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn space_then_identifier_then_space()
  {
    let mut lexer = Lexer::from_str(" foo ");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("foo")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn cr_then_identifier_then_cr()
  {
    let mut lexer = Lexer::from_str("\rfoo\r");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("foo")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn lf_then_identifier_then_lf()
  {
    let mut lexer = Lexer::from_str("\nfoo\n");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("foo")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn tab_then_identifier_then_tab()
  {
    let mut lexer = Lexer::from_str("\tfoo\t");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("foo")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn identifier_then_paren_l()
  {
    let mut lexer = Lexer::from_str("foo(");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("foo")));
    assert_eq!(lexer.next(), Some(Lexeme::symbol("(")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn identifier_then_paren_r()
  {
    let mut lexer = Lexer::from_str("foo)");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("foo")));
    assert_eq!(lexer.next(), Some(Lexeme::symbol(")")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn identifier_then_brace_l()
  {
    let mut lexer = Lexer::from_str("foo{");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("foo")));
    assert_eq!(lexer.next(), Some(Lexeme::symbol("{")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn identifier_then_brace_r()
  {
    let mut lexer = Lexer::from_str("foo}");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("foo")));
    assert_eq!(lexer.next(), Some(Lexeme::symbol("}")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn identifier_then_bracket_l()
  {
    let mut lexer = Lexer::from_str("foo[");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("foo")));
    assert_eq!(lexer.next(), Some(Lexeme::symbol("[")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn identifier_then_bracket_r()
  {
    let mut lexer = Lexer::from_str("foo]");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("foo")));
    assert_eq!(lexer.next(), Some(Lexeme::symbol("]")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn identifier_then_string()
  {
    let mut lexer = Lexer::from_str("foo`bar`");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("foo")));
    assert_eq!(lexer.next(), Some(Lexeme::string("bar")));
    assert_eq!(lexer.next(), None);
  }
}

#[cfg(test)]
mod reserved_words
{
  use super::super::*;

  #[test]
  fn val()
  {
    let mut lexer = Lexer::from_str("val");
    assert_eq!(lexer.next(), Some(Lexeme::keyword("val")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn val_()
  {
    let mut lexer = Lexer::from_str("val_");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("val_")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn def()
  {
    let mut lexer = Lexer::from_str("def");
    assert_eq!(lexer.next(), Some(Lexeme::keyword("def")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn def_()
  {
    let mut lexer = Lexer::from_str("def_");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("def_")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn fun()
  {
    let mut lexer = Lexer::from_str("fun");
    assert_eq!(lexer.next(), Some(Lexeme::keyword("fun")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn fun_()
  {
    let mut lexer = Lexer::from_str("fun_");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("fun_")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn sym_eq()
  {
    let mut lexer = Lexer::from_str("=");
    assert_eq!(lexer.next(), Some(Lexeme::keyword("=")));
  }

  #[test]
  fn sym_eq_()
  {
    let mut lexer = Lexer::from_str("=_");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("=_")));
  }

  #[test]
  fn _sym_eq()
  {
    let mut lexer = Lexer::from_str("_=");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("_=")));
  }

  #[test]
  fn r#true()
  {
    let mut lexer = Lexer::from_str("true");
    assert_eq!(lexer.next(), Some(Lexeme::keyword("true")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn r#true_()
  {
    let mut lexer = Lexer::from_str("true_");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("true_")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn r#false()
  {
    let mut lexer = Lexer::from_str("false");
    assert_eq!(lexer.next(), Some(Lexeme::keyword("false")));
    assert_eq!(lexer.next(), None);
  }

  #[test]
  fn r#false_()
  {
    let mut lexer = Lexer::from_str("false_");
    assert_eq!(lexer.next(), Some(Lexeme::identifier("false_")));
    assert_eq!(lexer.next(), None);
  }
}

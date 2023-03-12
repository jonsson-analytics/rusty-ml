mod backtracking_iterator;
#[macro_use]
mod can_backtrack;
mod declaration_parser;
mod expect_syntax;
mod expression_parser;
mod node_type;
mod parse_error;
mod with_backtracking;

pub use backtracking_iterator::*;
pub use can_backtrack::*;
pub use declaration_parser::*;
pub use expect_syntax::*;
pub use expression_parser::*;
pub use node_type::*;
pub use parse_error::*;
pub use with_backtracking::*;

use super::lexemes::Lexeme;
use super::tokens::Token;
use crate::syntax::surface;

pub type Result<T> = std::result::Result<T, ParseError>;

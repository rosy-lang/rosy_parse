pub mod common;

mod lexer;
mod parser;
mod reader;

use crate::common::error::R;
use crate::parser::ast::Ast;
use crate::parser::Parser;

pub fn parse<'a>(source: &'a str) -> R<Ast> {
	Parser::new(source).parse()
}

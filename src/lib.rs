mod common;
mod lexer;
mod parser;
mod reader;

use crate::common::result::ParseResult;
use crate::parser::Parser;

pub fn parse<'a>(source: &'a str, filename: &'a str) -> ParseResult<'a> {
	let mut parser = Parser::new(source);

	ParseResult::new(parser.parse(), source, filename)
}

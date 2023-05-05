mod common;
mod lexer;
mod parser;
mod reader;

use crate::common::result::ParseResult;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::reader::Reader;

pub fn parse<'a>(source: &'a str, filename: &'a str) -> ParseResult<'a> {
	let reader = Reader::new(source);
	let lexer = Lexer::new(reader);
	let mut parser = Parser::new(lexer);

	ParseResult::new(parser.parse_program(), source, filename)
}

pub mod lexer;
pub mod parser;

use crate::common::span::Span;

pub type R<T> = Result<T, ParseError>;

#[derive(Clone, Debug)]
pub struct ParseError {
	pub ty: String,
	pub labels: Vec<(String, Span)>,
	pub note: String,
}

pub mod lexer;
pub mod parser;

use crate::common::span::Span;

pub type R<T> = Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Error {
	pub ty: String,
	pub labels: Vec<(String, Span)>,
	pub note: String,
}

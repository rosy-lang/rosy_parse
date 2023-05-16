use crate::common::span::Span;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
	Eof,
	Boolean(bool),
	Integer(i64),
	Identifier(String),
	KwIf,
	KwThen,
	KwElse,
	KwWhile,
	KwDo,
	LParen,
	RParen,
	Comma,
	Colon,
	Equal,
	RArrow,
	Operator(String),
	Separator,
	BlockStart,
	BlockEnd,
}

impl TokenKind {
	pub fn name(&self) -> String {
		match self {
			TokenKind::Eof => String::from("end of file"),
			TokenKind::Boolean(b) => format!("boolean ({b})"),
			TokenKind::Integer(i) => format!("integer ({i})"),
			TokenKind::Identifier(i) => format!("identifier ({i})"),
			TokenKind::KwIf => String::from("keyword (if)"),
			TokenKind::KwThen => String::from("keyword (then)"),
			TokenKind::KwElse => String::from("keyword (else)"),
			TokenKind::KwWhile => String::from("keyword (while)"),
			TokenKind::KwDo => String::from("keyword (do)"),
			TokenKind::LParen => String::from("left parenthesis"),
			TokenKind::RParen => String::from("right parenthesis"),
			TokenKind::Comma => String::from("comma"),
			TokenKind::Colon => String::from("colon"),
			TokenKind::Equal => String::from("equal sign"),
			TokenKind::RArrow => String::from("right arrow"),
			TokenKind::Operator(op) => format!("operator ({op})"),
			TokenKind::Separator => String::from("newline"),
			TokenKind::BlockStart => String::from("start of block"),
			TokenKind::BlockEnd => String::from("end of block"),
		}
	}
}

#[derive(Debug)]
pub struct Token {
	pub kind: TokenKind,
	pub span: Span,
}

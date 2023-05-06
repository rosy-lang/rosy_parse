use crate::common::error::ParseError;
use crate::common::span::Span;
use crate::lexer::token::TokenKind;

pub fn invalid_definition(kind: &TokenKind, span1: Span, span2: Span) -> ParseError {
	let ty = String::from("invalid definition");

	let msg1 = String::from("identifier to be defined");
	let msg2 = format!("found: {}", kind.name());
	let labels = vec![(msg1, span1), (msg2, span2)];

	let note = format!(
		"identifier must be followed by {} or {}",
		TokenKind::Equal.name(),
		TokenKind::LParen.name(),
	);

	ParseError { ty, labels, note }
}

pub fn invalid_expr(kind: &TokenKind, span: Span) -> ParseError {
	let ty = String::from("invalid expression");

	let name = kind.name();
	let msg = format!("found: {}", name);
	let labels = vec![(msg, span)];

	let note = format!("expected expression, but found {}", name);

	ParseError { ty, labels, note }
}

pub fn invalid_identifier(kind: &TokenKind, span: Span) -> ParseError {
	let ty = String::from("invalid identifier");

	let name = kind.name();
	let msg = format!("found: {}", name);
	let labels = vec![(msg, span)];

	let note = format!("expected identifier, but found {}", name);

	ParseError { ty, labels, note }
}

pub fn invalid_infix_expr(kind: &TokenKind, span: Span) -> ParseError {
	let ty = String::from("invalid infix expression");

	let name = kind.name();
	let msg = format!("found: {}", name);
	let labels = vec![(msg, span)];

	let note = format!("expected infix expression, but found {}", name);

	ParseError { ty, labels, note }
}

pub fn unexpected_token(actual: &TokenKind, expected: &TokenKind, span: Span) -> ParseError {
	let ty = String::from("unexpected token");

	let actual_name = actual.name();
	let expected_name = expected.name();
	let msg = format!("found: {}", actual_name);
	let labels = vec![(msg, span)];

	let note = format!("expected {expected_name}, but found {actual_name}");

	ParseError { ty, labels, note }
}

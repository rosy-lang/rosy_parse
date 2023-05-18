use rosy_error::RosyError;

use crate::common::span::Span;
use crate::lexer::token::TokenKind;

pub fn invalid_declaration(kind: &TokenKind, span1: Span, span2: Span) -> RosyError {
	let title = String::from("invalid declaration");

	let msg1 = String::from("identifier to be defined");
	let msg2 = format!("found: {}", kind.name());
	let labels = vec![(msg1, span1.into()), (msg2, span2.into())];

	let description = format!(
		"identifier must be followed by {} for function definitions",
		TokenKind::LParen.name(),
	);

	RosyError {
		title,
		description,
		labels,
	}
}

pub fn invalid_expr(kind: &TokenKind, span: Span) -> RosyError {
	let title = String::from("invalid expression");

	let name = kind.name();
	let msg = format!("found: {}", name);
	let labels = vec![(msg, span.into())];

	let description = format!("expected expression, but found {}", name);

	RosyError {
		title,
		description,
		labels,
	}
}

pub fn invalid_identifier(kind: &TokenKind, span: Span) -> RosyError {
	let title = String::from("invalid identifier");

	let name = kind.name();
	let msg = format!("found: {}", name);
	let labels = vec![(msg, span.into())];

	let description = format!("expected identifier, but found {}", name);

	RosyError {
		title,
		description,
		labels,
	}
}

pub fn invalid_infix_expr(kind: &TokenKind, span: Span) -> RosyError {
	let title = String::from("invalid infix expression");

	let name = kind.name();
	let msg = format!("found: {}", name);
	let labels = vec![(msg, span.into())];

	let description = format!("expected infix expression, but found {}", name);

	RosyError {
		title,
		description,
		labels,
	}
}

pub fn invalid_type(kind: &TokenKind, span: Span) -> RosyError {
	let title = String::from("invalid type");

	let name = kind.name();
	let msg = format!("found: {}", name);
	let labels = vec![(msg, span.into())];

	let description = format!(
		"types must start with identifier or {}",
		TokenKind::LParen.name(),
	);

	RosyError {
		title,
		description,
		labels,
	}
}

pub fn unexpected_token(actual: &TokenKind, expected: &TokenKind, span: Span) -> RosyError {
	let title = String::from("unexpected token");

	let actual_name = actual.name();
	let expected_name = expected.name();
	let msg = format!("found: {}", actual_name);
	let labels = vec![(msg, span.into())];

	let description = format!("expected {expected_name}, but found {actual_name}");

	RosyError {
		title,
		description,
		labels,
	}
}

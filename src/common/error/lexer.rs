use rosy_error::RosyError;
use unicode_names2::name;

use crate::common::span::Span;

pub fn inconsistent_indent(indent: usize, span: Span) -> RosyError {
	let title = String::from("inconsistent indent");

	let msg = format!("indent level: {}", indent);
	let labels = vec![(msg, span.into())];

	let description = String::from("indent level does not match any of the previous indent levels");

	RosyError {
		title,
		description,
		labels,
	}
}

pub fn insufficient_indent(indent: usize, min_indent: usize, span: Span) -> RosyError {
	let title = String::from("insufficient indent");

	let msg = format!("indent level: {}", indent);
	let labels = vec![(msg, span.into())];

	let description = format!("indent level needs to be at least {min_indent}");

	RosyError {
		title,
		description,
		labels,
	}
}

pub fn unrecognized_character(c: char, span: Span) -> RosyError {
	let title = String::from("unrecognized character");

	let char_name = name(c).map(|n| n.to_string()).unwrap_or(String::new());
	let msg = format!("character: {char_name} ({:#x})", c as u32);
	let labels = vec![(msg, span.into())];

	let description = format!("character is not recognized as a valid identifier or symbol");

	RosyError {
		title,
		description,
		labels,
	}
}

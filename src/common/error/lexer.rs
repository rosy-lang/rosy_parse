use unicode_names2::name;

use crate::common::error::Error;
use crate::common::span::Span;

pub fn inconsistent_indent(indent: usize, span: Span) -> Error {
	let ty = String::from("inconsistent indent");

	let msg = format!("indent level: {}", indent);
	let labels = vec![(msg, span)];

	let note = String::from("indent level does not match any of the previous indent levels");

	Error { ty, labels, note }
}

pub fn insufficient_indent(indent: usize, min_indent: usize, span: Span) -> Error {
	let ty = String::from("insufficient indent");

	let msg = format!("indent level: {}", indent);
	let labels = vec![(msg, span)];

	let note = format!("indent level needs to be at least {min_indent}");

	Error { ty, labels, note }
}

pub fn unrecognized_character(c: char, span: Span) -> Error {
	let ty = String::from("unrecognized character");

	let char_name = name(c).map(|n| n.to_string()).unwrap_or(String::new());
	let msg = format!("character: {char_name} ({:#x})", c as u32);
	let labels = vec![(msg, span)];

	let note = format!("character is not recognized as a valid identifier or symbol");

	Error { ty, labels, note }
}

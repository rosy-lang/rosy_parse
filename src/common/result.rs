use ariadne::{Label, Report, ReportKind, Source};

use crate::common::error::{ParseError, R};
use crate::common::inspect::inspect_program;
use crate::parser::ast::*;

#[derive(Debug)]
pub struct ParseResult<'a> {
	pub value: R<Ast>,
	source: &'a str,
	filename: &'a str,
}

impl<'a> ParseResult<'a> {
	pub fn new(value: R<Ast>, source: &'a str, filename: &'a str) -> Self {
		Self {
			value,
			source,
			filename,
		}
	}

	pub fn print(&self) {
		match &self.value {
			Ok(program) => self.print_program(program),
			Err(err) => self.print_error(err),
		}
	}

	fn print_program(&self, program: &Ast) {
		println!("{}", inspect_program(program));
	}

	fn print_error(&self, error: &ParseError) {
		let ParseError { ty, labels, note } = error;

		let location = labels[0].1;

		let mut builder =
			Report::build(ReportKind::Error, self.filename, location.0).with_message(&ty);

		for label in labels {
			let (msg, span) = label;
			builder.add_label(Label::new((self.filename, span.0..span.1)).with_message(&msg));
		}

		builder
			.with_note(&note)
			.finish()
			.eprint((self.filename, Source::from(self.source)))
			.unwrap();
	}
}

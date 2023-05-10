use std::fmt::{Display, Formatter, Result};
use std::ops::Range;

#[derive(Clone, Copy, Debug)]
pub struct Span(pub usize, pub usize);

impl Span {
	pub fn new(start: usize, end: usize) -> Self {
		Self(start, end)
	}

	pub fn pair(index: usize) -> Self {
		Self::new(index, index)
	}

	pub fn between(start: Span, end: Span) -> Self {
		Self(start.0, end.1)
	}
}

impl Display for Span {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}..{}", self.0, self.1)
	}
}

impl From<Span> for Range<usize> {
	fn from(span: Span) -> Range<usize> {
		span.0..span.1
	}
}

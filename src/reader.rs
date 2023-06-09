use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Reader {
	pub index: usize,
	pub ln: usize,
	pub col: usize,
	chars: Peekable<IntoIter<char>>,
}

impl Reader {
	pub fn new(source: &str) -> Self {
		// TODO: optimize memory usage
		let chars = String::from(source).chars().collect::<Vec<_>>();

		let reader = Self {
			index: 0,
			ln: 1,
			col: 1,
			chars: chars.into_iter().peekable(),
		};

		reader
	}

	pub fn skip_whitespace(&mut self, include_newline: bool) {
		loop {
			if !self.peek().is_ascii_whitespace() {
				return;
			}

			if !include_newline && self.peek() == '\n' {
				return;
			}

			self.next();
		}
	}

	pub fn peek(&mut self) -> char {
		match self.chars.peek() {
			Some(c) => c.clone(),
			None => '\0',
		}
	}

	pub fn next(&mut self) -> char {
		let c = self.peek();
		self.chars.next();

		if c == '\n' {
			self.ln += 1;
			self.col = 1;
		} else {
			self.col += 1;
		}

		// TODO: change this to account for codepoint sizes, if necessary
		self.index += 1;

		c
	}

	pub fn eof(&mut self) -> bool {
		self.peek() == '\0'
	}
}

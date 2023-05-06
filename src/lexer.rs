pub mod token;

use std::collections::VecDeque;

use crate::common::error::lexer::*;
use crate::common::error::R;
use crate::common::span::Span;
use crate::lexer::token::{Token, TokenKind};
use crate::reader::Reader;

pub struct Lexer {
	pub input: Reader,
	buffer: VecDeque<Token>,
	indents: Vec<usize>,
}

impl Lexer {
	pub fn new(input: Reader) -> Self {
		Self {
			input,
			buffer: VecDeque::new(),
			indents: Vec::new(),
		}
	}

	pub fn generate(&mut self) -> R<()> {
		let c = self.input.peek();
		let start = self.input.index;

		if start == 0 || c == '\n' {
			self.detect_indent()?;
			self.generate()?;
		} else if c.is_ascii_whitespace() {
			self.input.skip_whitespace(false);
			self.generate()?;
		} else if self.is_identifier_start() {
			let mut lexeme = String::new();
			while self.is_identifier() {
				lexeme.push(self.input.next());
			}

			let is_integer = lexeme.chars().all(|c| c.is_ascii_digit());
			let mut is_layout_token = false;

			let kind = match lexeme.as_str() {
				"true" => TokenKind::Boolean(true),
				"false" => TokenKind::Boolean(false),
				"if" => {
					is_layout_token = true;
					TokenKind::KwIf
				},
				"then" => {
					is_layout_token = true;
					TokenKind::KwThen
				},
				"else" => {
					is_layout_token = true;
					TokenKind::KwElse
				},
				"while" => {
					is_layout_token = true;
					TokenKind::KwWhile
				},
				"do" => {
					is_layout_token = true;
					TokenKind::KwDo
				},
				_ if is_integer => TokenKind::Integer(lexeme.parse().unwrap()),
				_ => TokenKind::Identifier(lexeme),
			};

			let end = self.input.index;

			let token = Token {
				kind,
				span: Span::new(start, end),
			};

			self.buffer.push_back(token);

			if is_layout_token {
				self.prepare_block()?;
			}
		} else if self.is_symbol() {
			let kind = match self.input.next() {
				'(' => TokenKind::LParen,
				')' => TokenKind::RParen,
				',' => TokenKind::Comma,
				_ => unreachable!(),
			};

			let end = self.input.index;

			let token = Token {
				kind,
				span: Span::new(start, end),
			};

			self.buffer.push_back(token);
		} else if self.is_operator() {
			let mut lexeme = String::new();
			while self.is_operator() {
				lexeme.push(self.input.next());
			}

			let mut is_layout_token = true;
			let kind = match lexeme.as_str() {
				"=" => TokenKind::Equal,
				"->" => TokenKind::RArrow,
				_ => {
					is_layout_token = false;
					TokenKind::Operator(lexeme)
				},
			};

			let end = self.input.index;

			let token = Token {
				kind,
				span: Span::new(start, end),
			};

			self.buffer.push_back(token);

			if is_layout_token {
				self.prepare_block()?;
			}
		} else if self.input.eof() {
			while self.indents.len() > 1 {
				let token = Token {
					kind: TokenKind::BlockEnd,
					span: Span::pair(self.input.index),
				};

				self.buffer.push_back(token);
				self.indents.pop();
			}

			let token = Token {
				kind: TokenKind::Eof,
				span: Span::pair(self.input.index),
			};

			self.buffer.push_back(token);
		} else {
			self.input.next();

			let end = self.input.index;
			let span = Span::new(start, end);

			return Err(unrecognized_character(c, span));
		}

		Ok(())
	}

	fn prepare_block(&mut self) -> R<()> {
		macro_rules! col {
			() => {
				self.input.col
			};
		}

		let start_ln = self.input.ln;
		self.input.skip_whitespace(true);
		if self.input.eof() {
			return Ok(());
		}

		let end_ln = self.input.ln;

		let indent = self.indents[self.indents.len() - 1];
		if col!() <= indent {
			let start = self.input.index - col!() + 1;
			let end = self.input.index;

			let span = Span::new(start, end);

			return Err(insufficient_indent(col!() - 1, indent, span))
		}

		if start_ln != end_ln {
			let token = Token {
				kind: TokenKind::BlockStart,
				span: Span::pair(self.input.index),
			};

			self.buffer.push_back(token);
			self.indents.push(col!());
		}

		Ok(())
	}

	fn detect_indent(&mut self) -> R<()> {
		macro_rules! col {
			() => {
				self.input.col
			};
		}

		macro_rules! indent {
			() => {
				self.indents[self.indents.len() - 1]
			};
		}

		self.input.skip_whitespace(true);
		if self.input.eof() {
			return Ok(());
		}

		if self.indents.is_empty() {
			self.indents.push(1);
			return Ok(())
		}

		if col!() < indent!() {
			while col!() < indent!() {
				let token = Token {
					kind: TokenKind::BlockEnd,
					span: Span::pair(self.input.index),
				};

				self.buffer.push_back(token);
				self.indents.pop();
			}

			if col!() != indent!() {
				let start = self.input.index - col!() + 1;
				let end = self.input.index;

				let span = Span::new(start, end);
				return Err(inconsistent_indent(col!() - 1, span));
			}
		}

		if col!() == indent!() {
			let token = Token {
				kind: TokenKind::Separator,
				span: Span::pair(self.input.index),
			};

			self.buffer.push_back(token);
		}

		Ok(())
	}

	fn is_identifier(&mut self) -> bool {
		let c = self.input.peek();
		self.is_identifier_start() || "!'?".contains(c)
	}

	fn is_identifier_start(&mut self) -> bool {
		let c = self.input.peek();
		c.is_ascii_alphanumeric() || c == '_'
	}

	fn is_symbol(&mut self) -> bool {
		let c = self.input.peek();
		"(),".contains(c)
	}

	fn is_operator(&mut self) -> bool {
		let c = self.input.peek();
		"!*+-/<=>".contains(c)
	}

	pub fn peek(&mut self) -> R<&Token> {
		if self.buffer.is_empty() {
			self.generate()?;
		}

		let token = self.buffer.front().unwrap();

		Ok(token)
	}

	pub fn next(&mut self) -> R<Token> {
		if self.buffer.is_empty() {
			self.generate()?;
		}

		let token = self.buffer.pop_front().unwrap();

		Ok(token)
	}

	pub fn restore(&mut self, token: Token) {
		self.buffer.push_front(token);
	}

	pub fn eof(&mut self) -> bool {
		if let Ok(t) = self.peek() {
			t.kind == TokenKind::Eof
		} else {
			false
		}
	}
}

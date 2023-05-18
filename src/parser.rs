pub mod ast;

use crate::common::error::parser::*;
use crate::common::error::R;
use crate::common::span::Span;
use crate::lexer::token::{Token, TokenKind};
use crate::lexer::Lexer;
use crate::parser::ast::*;
use crate::reader::Reader;

pub struct Parser {
	lexer: Lexer,
}

impl Parser {
	pub fn new(lexer: &str) -> Self {
		let reader = Reader::new(lexer);
		let lexer = Lexer::new(reader);

		Self { lexer }
	}

	pub fn parse(&mut self) -> R<Ast> {
		let mut ast = Vec::new();

		while !self.lexer.eof() {
			let decl = self.parse_decl()?;
			ast.push(decl);

			self.discard(TokenKind::Separator)?;
		}

		Ok(ast)
	}

	fn parse_decl(&mut self) -> R<Decl> {
		let identifier = self.parse_identifier()?;
		let span = self.span()?;

		match self.kind()? {
			TokenKind::LParen => {
				let fn_def = self.parse_fn_def(identifier)?;
				let span = fn_def.span;

				let decl = Decl {
					kind: DeclKind::Fn(fn_def),
					span,
				};

				Ok(decl)
			},
			TokenKind::Colon => {
				let ty_decl = self.parse_ty_decl(identifier)?;
				let span = ty_decl.span;

				let decl = Decl {
					kind: DeclKind::Ty(ty_decl),
					span,
				};

				Ok(decl)
			},
			kind => Err(invalid_declaration(kind, identifier.span, span)),
		}
	}

	fn parse_stmt(&mut self) -> R<Stmt> {
		let stmt = if matches!(self.kind()?, TokenKind::Identifier(_)) {
			let identifier = self.parse_identifier()?;

			if self.kind_is(TokenKind::Colon) {
				let ty_decl = self.parse_ty_decl(identifier)?;
				let span = ty_decl.span;

				Stmt {
					kind: StmtKind::TyDecl(ty_decl),
					span,
				}
			} else if self.kind_is(TokenKind::Equal) {
				let var_def = self.parse_var_def(identifier)?;
				let span = var_def.span;

				Stmt {
					kind: StmtKind::VarDef(var_def),
					span,
				}
			} else {
				let span = identifier.span;

				let expr = Expr {
					kind: ExprKind::Identifier(identifier.name),
					span,
				};

				let expr = self.parse_infix(expr, 0)?;

				Stmt {
					kind: StmtKind::Expr(expr),
					span,
				}
			}
		} else {
			let expr = self.parse_expr(0)?;
			let span = expr.span;

			Stmt {
				kind: StmtKind::Expr(expr),
				span,
			}
		};

		Ok(stmt)
	}

	fn parse_ty_decl(&mut self, identifier: Identifier) -> R<TyDecl> {
		self.consume(TokenKind::Colon)?;

		let ty = self.parse_ty()?;

		let start = identifier.span;
		let end = ty.span;

		let ty_decl = TyDecl {
			identifier,
			ty,
			span: Span::between(start, end),
		};

		Ok(ty_decl)
	}

	fn parse_fn_def(&mut self, func: Identifier) -> R<FnDef> {
		self.consume(TokenKind::LParen)?;

		let mut params = Vec::new();

		while !self.kind_is(TokenKind::RParen) {
			let identifier = self.parse_identifier()?;
			params.push(identifier);

			self.discard(TokenKind::Comma)?;
		}

		self.consume(TokenKind::RParen)?;
		self.consume(TokenKind::Equal)?;

		let expr = self.parse_expr(0)?;

		let start = func.span;
		let end = expr.span;

		let fn_def = FnDef {
			func,
			params,
			body: expr,
			span: Span::between(start, end),
		};

		Ok(fn_def)
	}

	fn parse_var_def(&mut self, identifier: Identifier) -> R<VarDef> {
		self.consume(TokenKind::Equal)?;

		let expr = self.parse_expr(0)?;

		let start = identifier.span;
		let end = expr.span;

		let var_def = VarDef {
			var: identifier,
			value: expr,
			span: Span::between(start, end),
		};

		Ok(var_def)
	}

	fn parse_expr(&mut self, prec: usize) -> R<Expr> {
		let start = self.span()?;

		let lhs = match self.kind()? {
			TokenKind::Boolean(b) => {
				let kind = ExprKind::Boolean(*b);
				self.lexer.next()?;

				Expr { kind, span: start }
			},
			TokenKind::Integer(i) => {
				let kind = ExprKind::Integer(*i);
				self.lexer.next()?;

				Expr { kind, span: start }
			},
			TokenKind::Identifier(i) => {
				let kind = ExprKind::Identifier(i.clone());
				self.lexer.next()?;

				Expr { kind, span: start }
			},
			TokenKind::KwIf => {
				self.consume(TokenKind::KwIf)?;

				let cond = self.parse_expr(0)?;

				self.discard(TokenKind::Separator)?;
				self.consume(TokenKind::KwThen)?;

				let then_expr = self.parse_expr(0)?;
				let mut end = then_expr.span;

				if self.kind_is(TokenKind::Separator) {
					let sep = self.consume(TokenKind::Separator)?;

					if !self.kind_is(TokenKind::KwElse) {
						self.lexer.restore(sep);
					}
				}

				let else_expr = if self.kind_is(TokenKind::KwElse) {
					self.consume(TokenKind::KwElse)?;

					let else_expr = self.parse_expr(0)?;
					end = else_expr.span;

					Some(Box::new(else_expr))
				} else {
					None
				};

				Expr {
					kind: ExprKind::If(Box::new(cond), Box::new(then_expr), else_expr),
					span: Span::between(start, end),
				}
			},
			TokenKind::KwWhile => {
				self.consume(TokenKind::KwWhile)?;

				let cond = self.parse_expr(0)?;

				self.discard(TokenKind::Separator)?;
				self.consume(TokenKind::KwDo)?;

				let expr = self.parse_expr(0)?;
				let end = expr.span;

				Expr {
					kind: ExprKind::While(Box::new(cond), Box::new(expr)),
					span: Span::between(start, end),
				}
			},
			TokenKind::LParen => {
				self.consume(TokenKind::LParen)?;

				let expr = self.parse_expr(0)?;

				let token = self.consume(TokenKind::RParen)?;
				let end = token.span;

				Expr {
					kind: expr.kind,
					span: Span::between(start, end),
				}
			},
			TokenKind::Operator(_) => {
				let token = self.lexer.next()?;

				let TokenKind::Operator(op) = token.kind else {
					unreachable!();
				};

				let func = Expr {
					kind: ExprKind::Identifier(op),
					span: start,
				};

				let arg = self.parse_expr(0)?;
				let end = arg.span;

				Expr {
					kind: ExprKind::Call(Box::new(func), vec![arg]),
					span: Span::between(start, end),
				}
			},
			TokenKind::BlockStart => {
				self.consume(TokenKind::BlockStart)?;

				let mut stmts = Vec::new();

				while !self.kind_is(TokenKind::BlockEnd) {
					let stmt = self.parse_stmt()?;
					stmts.push(stmt);

					self.discard(TokenKind::Separator)?;
				}

				let token = self.consume(TokenKind::BlockEnd)?;
				let end = token.span;

				Expr {
					kind: ExprKind::Block(stmts),
					span: Span::between(start, end),
				}
			},
			kind => return Err(invalid_expr(kind, start)),
		};

		let expr = self.parse_infix(lhs, prec)?;

		Ok(expr)
	}

	fn parse_infix(&mut self, mut lhs: Expr, prec: usize) -> R<Expr> {
		let start = lhs.span;

		loop {
			let span = self.span()?;

			match self.kind()? {
				TokenKind::LParen => {
					self.consume(TokenKind::LParen)?;

					let mut args = Vec::new();

					while !self.kind_is(TokenKind::RParen) {
						let arg = self.parse_expr(0)?;
						args.push(arg);

						self.discard(TokenKind::Comma)?;
					}

					let token = self.consume(TokenKind::RParen)?;
					let end = token.span;

					lhs = Expr {
						kind: ExprKind::Call(Box::new(lhs), args),
						span: Span::between(start, end),
					};
				},
				TokenKind::Operator(op) => {
					let bin_op = BinaryOp {
						lexeme: op.clone(),
						span,
					};

					match bin_op.assoc() {
						OpAssoc::Left if bin_op.prec() <= prec => break,
						OpAssoc::Right if bin_op.prec() < prec => break,
						_ => (),
					}

					self.lexer.next()?;

					let rhs = self.parse_expr(bin_op.prec())?;
					let end = rhs.span;

					let func = Expr {
						kind: ExprKind::Identifier(bin_op.lexeme),
						span: bin_op.span,
					};

					lhs = Expr {
						kind: ExprKind::Call(Box::new(func), vec![lhs, rhs]),
						span: Span::between(start, end),
					};
				},
				TokenKind::Eof
				| TokenKind::KwThen
				| TokenKind::KwElse
				| TokenKind::KwDo
				| TokenKind::RParen
				| TokenKind::Comma
				| TokenKind::Separator
				| TokenKind::BlockEnd => break,
				kind => return Err(invalid_infix_expr(kind, span)),
			};
		}

		Ok(lhs)
	}

	fn parse_ty(&mut self) -> R<Ty> {
		let start = self.span()?;

		let mut lhs = match self.kind()? {
			TokenKind::Identifier(i) => {
				let kind = TyKind::Single(i.clone());
				self.lexer.next()?;

				Ty { kind, span: start }
			},
			TokenKind::LParen => {
				self.consume(TokenKind::LParen)?;

				let mut tys = Vec::new();

				while !self.kind_is(TokenKind::RParen) {
					let ty = self.parse_ty()?;
					tys.push(ty);

					self.discard(TokenKind::Comma)?;
				}

				let token = self.consume(TokenKind::RParen)?;
				let end = token.span;

				Ty {
					kind: TyKind::Tuple(tys),
					span: Span::between(start, end),
				}
			},
			kind => return Err(invalid_type(kind, start)),
		};

		if self.kind_is(TokenKind::RArrow) {
			self.consume(TokenKind::RArrow)?;

			let rhs = self.parse_ty()?;
			let end = rhs.span;

			let kind = match lhs.kind {
				TyKind::Tuple(tys) => TyKind::Function(tys, Box::new(rhs)),
				_ => TyKind::Function(vec![lhs], Box::new(rhs)),
			};

			lhs = Ty {
				kind,
				span: Span::between(start, end),
			}
		}

		Ok(lhs)
	}

	fn parse_identifier(&mut self) -> R<Identifier> {
		let token = self.lexer.next()?;

		match token.kind {
			TokenKind::Identifier(name) => {
				let identifier = Identifier {
					name,
					span: token.span,
				};

				Ok(identifier)
			},
			kind => Err(invalid_identifier(&kind, token.span)),
		}
	}

	fn discard(&mut self, kind: TokenKind) -> R<()> {
		if self.kind_is(kind) {
			self.lexer.next()?;
		}

		Ok(())
	}

	fn consume(&mut self, kind: TokenKind) -> R<Token> {
		let span = self.span()?;
		let actual_kind = self.kind()?;

		if *actual_kind == kind {
			self.lexer.next()
		} else {
			Err(unexpected_token(actual_kind, &kind, span))
		}
	}

	fn kind_is(&mut self, kind: TokenKind) -> bool {
		match self.kind() {
			Ok(k) => *k == kind,
			Err(_) => false,
		}
	}

	fn kind(&mut self) -> R<&TokenKind> {
		self.lexer.peek().map(|t| &t.kind)
	}

	fn span(&mut self) -> R<Span> {
		self.lexer.peek().map(|t| t.span)
	}
}

use crate::common::span::Span;

pub type Ast = Vec<Decl>;

#[derive(Debug)]
pub struct Decl {
	pub kind: DeclKind,
	pub span: Span,
}

#[derive(Debug)]
pub enum DeclKind {
	Ty(TyDecl),
	Fn(FnDef),
}

#[derive(Debug)]
pub struct Stmt {
	pub kind: StmtKind,
	pub span: Span,
}

#[derive(Debug)]
pub enum StmtKind {
	TyDecl(TyDecl),
	VarDef(VarDef),
	Expr(Expr),
}

#[derive(Debug)]
pub struct Expr {
	pub kind: ExprKind,
	pub span: Span,
}

#[derive(Debug)]
pub enum ExprKind {
	Boolean(bool),
	Integer(i64),
	Identifier(String),
	If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
	While(Box<Expr>, Box<Expr>),
	Call(Box<Expr>, Vec<Expr>),
	Block(Vec<Stmt>),
}

#[derive(Debug)]
pub struct Ty {
	pub kind: TyKind,
	pub span: Span,
}

#[derive(Debug)]
pub enum TyKind {
	Single(String),
	Tuple(Vec<Ty>),
	Function(Vec<Ty>, Box<Ty>),
}

#[derive(Debug)]
pub struct TyDecl {
	pub identifier: Identifier,
	pub ty: Ty,
	pub span: Span,
}

#[derive(Debug)]
pub struct FnDef {
	pub func: Identifier,
	pub params: Vec<Identifier>,
	pub body: Expr,
	pub span: Span,
}

#[derive(Debug)]
pub struct VarDef {
	pub var: Identifier,
	pub value: Expr,
	pub span: Span,
}

#[derive(Debug)]
pub struct Identifier {
	pub name: String,
	pub span: Span,
}

#[derive(Debug)]
pub struct BinaryOp {
	pub lexeme: String,
	pub span: Span,
}

impl BinaryOp {
	pub fn assoc(&self) -> OpAssoc {
		match self.lexeme.as_str() {
			"**" => OpAssoc::Right,
			_ => OpAssoc::Left,
		}
	}

	pub fn prec(&self) -> usize {
		match self.lexeme.as_str() {
			"||" => 1,
			"&&" => 2,
			"==" | "!=" => 3,
			"<" | "<=" | ">" | ">=" => 4,
			"+" | "-" => 5,
			"*" | "/" | "%" => 6,
			"**" => 7,
			_ => 8,
		}
	}
}

#[derive(Debug)]
pub enum OpAssoc {
	Left,
	Right,
}

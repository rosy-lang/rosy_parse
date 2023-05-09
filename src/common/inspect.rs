use indoc::formatdoc;

use crate::parser::ast::*;

pub fn inspect_program(ast: &Ast) -> String {
	let mut lines = vec![String::from("╰ AST")];

	for i in 0..ast.len() {
		let prefix = String::from("  ");
		let is_last = i == ast.len() - 1;

		let str = match &ast[i].kind {
			DefKind::Var(var_def) => inspect_var_def(var_def, prefix, is_last),
			DefKind::Fn(fn_def) => inspect_fn_def(fn_def, prefix, is_last),
		};

		lines.push(str);
	}

	lines.join("\n")
}

fn inspect_var_def(var_def: &VarDef, prefix: String, is_last: bool) -> String {
	let (p1, p2) = prefixes(prefix, is_last);

	let VarDef { var, value, span } = var_def;

	formatdoc! {"
		{p1}Define
		{p2}│╰ {span}
		{p2}├ Variable
		{}
		{p2}╰ Value
		{}"
	,
		inspect_identifier(&var, format!("{p2}│ "), true),
		inspect_expr(&value, format!("{p2}  "), true),
	}
}

fn inspect_fn_def(fn_def: &FnDef, prefix: String, is_last: bool) -> String {
	let (p1, p2) = prefixes(prefix, is_last);

	let FnDef {
		func,
		params,
		body,
		span,
	} = fn_def;

	let mut inspect_params: Vec<String> = Vec::new();

	for i in 0..params.len() {
		let prefix = format!("{p2}│ ");
		let is_last = i == params.len() - 1;

		let str = inspect_identifier(&params[i], prefix, is_last);

		inspect_params.push(str);
	}

	formatdoc! {"
		{p1}Define
		{p2}│╰ {span}
		{p2}├ Function
		{}
		{p2}├ Parameters
		{}
		{p2}╰ Body
		{}"
	,
		inspect_identifier(&func, format!("{p2}│ "), true),
		if inspect_params.len() == 0 {
			format!("{p2}│")
		} else {
			inspect_params.join("\n")
		},
		inspect_expr(&body, format!("{p2}  "), true),
	}
}

fn inspect_expr(expr: &Expr, prefix: String, is_last: bool) -> String {
	let (p1, p2) = prefixes(prefix, is_last);

	let Expr { kind, span } = expr;

	match kind {
		ExprKind::Boolean(b) => formatdoc! {"
			{p1}Boolean({b})
			{p2} ╰ {span}"
		},
		ExprKind::Integer(i) => formatdoc! {"
			{p1}Integer({i})
			{p2} ╰ {span}"
		},
		ExprKind::Identifier(i) => formatdoc! {"
			{p1}Identifier({})
			{p2} ╰ {span}"
		,
			i.name,
		},
		ExprKind::If(cond, t, e) => {
			if let Some(e) = e {
				formatdoc! {"
					{p1}If
					{p2}│╰ {span}
					{p2}├ Condition
					{}
					{p2}├ Then
					{}
					{p2}╰ Else
					{}"
				,
					inspect_expr(&cond, format!("{p2}│ "), true),
					inspect_expr(&t, format!("{p2}│ "), true),
					inspect_expr(&e, format!("{p2}  "), true),
				}
			} else {
				formatdoc! {"
					{p1}If
					{p2}│╰ {span}
					{p2}├ Condition
					{}
					{p2}╰ Then
					{}"
				,
					inspect_expr(&cond, format!("{p2}│ "), true),
					inspect_expr(&t, format!("{p2}  "), true),
				}
			}
		},
		ExprKind::While(cond, expr) => {
			formatdoc! {"
				{p1}While
				{p2}│╰ {span}
				{p2}├ Condition
				{}
				{p2}╰ Body
				{}"
			,
				inspect_expr(&cond, format!("{p2}│ "), true),
				inspect_expr(&expr, format!("{p2}  "), true),
			}
		},
		ExprKind::Call(func, args) => {
			let mut inspect_args: Vec<String> = Vec::new();

			for i in 0..args.len() {
				let prefix = format!("{p2}  ");
				let is_last = i == args.len() - 1;

				let str = inspect_expr(&args[i], prefix, is_last);

				inspect_args.push(str);
			}

			formatdoc! {"
				{p1}Call
				{p2}│╰ {span}
				{p2}├ Function
				{}
				{p2}╰ Arguments
				{}"
			,
				inspect_expr(&func, format!("{p2}│ "), true),
				if inspect_args.len() == 0 {
					format!("{p2}")
				} else {
					inspect_args.join("\n")
				},
			}
		},
		ExprKind::Block(stmts) => {
			let mut inspect_stmts: Vec<String> = Vec::new();

			for i in 0..stmts.len() {
				let prefix = p2.clone();
				let is_last = i == stmts.len() - 1;

				let str = match &stmts[i].kind {
					StmtKind::VarDef(var_def) => inspect_var_def(var_def, prefix, is_last),
					StmtKind::Expr(expr) => inspect_expr(expr, prefix, is_last),
				};

				inspect_stmts.push(str);
			}

			formatdoc! {"
				{p1}Block
				{p2}│╰ {span}
				{}",
				inspect_stmts.join("\n"),
			}
		},
	}
}

fn inspect_identifier(identifier: &Identifier, prefix: String, is_last: bool) -> String {
	let (p1, p2) = prefixes(prefix, is_last);

	let Identifier { name, span } = identifier;

	formatdoc! {"
		{p1}Identifier({name})
		{p2} ╰ {span}"
	}
}

fn prefixes(prefix: String, is_last: bool) -> (String, String) {
	let p1 = if is_last {
		format!("{prefix}╰ ")
	} else {
		format!("{prefix}├ ")
	};

	let p2 = if is_last {
		format!("{prefix}  ")
	} else {
		format!("{prefix}│ ")
	};

	(p1, p2)
}

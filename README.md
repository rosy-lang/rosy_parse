# rosy_parse
A library to parse Rosy source code

## Usage
```rust
use rosy_parse::parse;

fn main() {
	let source = include_str!("path/to/main.rosy");
	let filename = "main.rosy";

	let parse_result = parse(source, filename);
	parse_result.print();

	match parse_result.value {
		Ok(ast) => todo!(),
		Err(err) => todo!(),
	}
}
```

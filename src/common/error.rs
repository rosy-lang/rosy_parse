pub mod lexer;
pub mod parser;

use rosy_error::RosyError;

pub type R<T> = Result<T, RosyError>;

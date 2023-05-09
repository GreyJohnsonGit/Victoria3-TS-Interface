pub mod country_definition;
pub mod paradox_readable;
pub mod parser;
pub mod lexer_token;
pub mod extra;
pub mod format;

pub fn message() -> &'static str {
  return country_definition::message();
}


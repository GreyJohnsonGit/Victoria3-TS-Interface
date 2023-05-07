#[derive(PartialEq, Eq, Clone, Copy)]
pub enum LexerToken {
  Equals,
  Quote,
  LeftCurly,
  RightCurly,
  LeftParenthesis,
  RightParenthesis,
  Comment,
  Comma,
  Untyped
}

pub const EQUALS: char = '=';
pub const QUOTE: char = '"';
pub const LEFT_CURLY: char = '{';
pub const RIGHT_CURLY: char = '}';
pub const LEFT_PARENTHESIS: char = '(';
pub const RIGHT_PARENTHESIS: char = ')';
pub const POUND: char = '#';
pub const BANG: char = '!';
pub const COMMA: char = ',';
pub const SEMI_COLON: char = ';';

impl LexerToken {
  pub fn from_char(c: char) -> LexerToken {
    match c {
      EQUALS => LexerToken::Equals,
      QUOTE => LexerToken::Quote,
      LEFT_CURLY => LexerToken::LeftCurly,
      RIGHT_CURLY => LexerToken::RightCurly,
      LEFT_PARENTHESIS => LexerToken::LeftParenthesis,
      RIGHT_PARENTHESIS => LexerToken::RightParenthesis,
      POUND => LexerToken::Comment,
      BANG => LexerToken::Comment,
      COMMA => LexerToken::Comma,
      SEMI_COLON => LexerToken::Comma,
      _ => LexerToken::Untyped
    }
  }
}
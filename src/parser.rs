use std::str::FromStr;

use crate::format::Format;
use crate::paradox_readable::ParadoxReadable;
use chrono::NaiveDate;
use crate::lexer_token::*;

const MaxTokenSize: usize = 256;
const NumberStyles: usize = 0x00000000;
const MaxByteBuffer: usize = 1024 * 64;
// private const NumberStyles SignedFloatingStyle = NumberStyles.AllowDecimalPoint | NumberStyles.AllowLeadingSign;
const BufferSize: usize = 0x00000000;
// private static readonly int BufferSize = Globals.ParadoxEncoding.GetMaxCharCount(MaxByteBuffer);

pub trait IParadoxParser {
  // Getters
  fn get_current_indent(&self) -> usize;
  fn get_current_string(&self) -> String;
  fn is_end_of_stream(&self) -> bool;
  
  // Public Static
  fn deserialize<T>(data: Vec<u8>) -> T;
  fn parse<'a, T: ParadoxReadable>(data: Vec<u8>, entity: &'a mut T);
  
  // Parse
  fn parse_inner<'a, T: ParadoxReadable>(&mut self, inner_structure: &'a mut T) -> &'a mut T;
  
  // Read Fundamental
  fn read_string(&mut self) -> Option<String>;
  fn read_i32(&mut self) -> i32;
  fn read_u32(&mut self) -> u32;
  fn read_f32(&mut self) -> f32;
  fn read_f64(&mut self) -> f64;
  fn read_bool(&mut self) -> bool;
  fn read_date(&mut self) -> NaiveDate;
  
  // Read Advanced
  fn read_i32_vec(&self) -> Vec<i32>;
  fn read_f64_list(&self) -> Vec<f64>;
  fn read_string_list(&self) -> Vec<String>;
  fn read_date_list(&self) -> Vec<NaiveDate>;
  fn read_inside_brackets(&self, callback: fn(ParadoxParser));
  fn read_list<T>(&self, read_action: fn() -> T) -> Vec<T>;
  fn read_map<TKey, TValue>(
    &self, 
    key_mapping: fn(ParadoxParser) -> TKey,
    value_mapping: fn(ParadoxParser) -> TValue
  ) -> std::collections::HashMap<TKey, TValue>;
  
  // Controls
  fn ensure_left_curly(&mut self);
  fn next_is_bracketed(&mut self) -> bool;
  fn read_next(&mut self) -> char;
  
  // Mine
  fn current_token(&self) -> LexerToken;
  fn next_token(&self) -> Option<LexerToken>;
  fn consume_char(&mut self) -> char;
  fn consume_token(&mut self) -> LexerToken;
}

pub struct ParadoxParser {
  current_indent: usize,
  
  current_char: char,
  next_chars: Vec<char>,
  next_chars_empty: bool,
  
  buffer_index: usize,
  buffer_size: usize,
  buffer: [char; BufferSize],
  
  string_buffer: [char; MaxTokenSize],
  string_buffer_count: usize,
  current_string: String,
  
  eof: bool,
  tag_is_bracketed: Option<bool>,
}

impl IParadoxParser for ParadoxParser {
  fn get_current_indent(&self) -> usize { self.current_indent }
  fn get_current_string(&self) -> String { self.current_string.clone() }
  fn is_end_of_stream(&self) -> bool { self.eof }
  
  fn deserialize<T>(data: Vec<u8>) -> T {
    todo!()
  }
  
  fn parse<'a, T: ParadoxReadable>(data: Vec<u8>, entity: &'a mut T) {
    let mut parser = ParadoxParser::new();
    while !parser.is_end_of_stream() {
      let value = parser.read_string().unwrap_or_default();
      if value != "\0" {
        entity.pdx_token_callback(value);
      }
    }
  }
  
  fn read_string(&mut self) -> Option<String> {
    if 
    self.current_token() == LexerToken::LeftCurly && 
    self.next_token()
      .map(|token| token == LexerToken::RightCurly)
      .unwrap_or(false)  
    {
      return None;
    }
    
    let is_unquoted_character = |c: char| -> bool { 
      Format::is_not_space(c) &&
      LexerToken::from_char(c) == LexerToken::Untyped &&
      !self.eof
    };

    let is_unquoted_string = self.current_token() == LexerToken::Untyped;
    if is_unquoted_string {
      while is_unquoted_character(self.current_char) {
        self.add_to_string_buffer(self.current_char);
        self.advance_current_char();
      };

      return Some(self.consume_string_buffer());
    }
    
    let is_quoted_string = self.current_token() == LexerToken::Quote;
    if is_quoted_string {
      let open_quote = self.advance_current_char();
      while self.current_token() != LexerToken::Quote && !self.eof {
        self.add_to_string_buffer(self.current_char);
        self.advance_current_char();
      }
      let close_quote = self.advance_current_char();

      if self.current_char == UNDERSCORE {
        while is_unquoted_character(self.current_char) {
          self.add_to_string_buffer(self.current_char);
          self.advance_current_char();
        }
      }

      return Some(self.consume_string_buffer());
    }

    return self.read_string();
  }
  
  fn read_i32(&mut self) -> i32 {
    let mut result: i32 = 0;
    let mut sign = 1;
    
    self.consume_until_untyped();
    if self.eof { return 0; }

    while 
      Format::is_not_space(self.current_char) && 
      LexerToken::from_char(self.current_char) == LexerToken::Untyped 
    {
      if Format::is_digit(self.current_char) {
        result = result * 10 + (self.current_char as u8 - b'0') as i32;
      }
      
      if self.current_char == '-' {
        sign = -1;
      }

      self.advance_current_char();
    }

    return sign * result;
  }
  
  fn read_u32(&mut self) -> u32 {
    let mut result: u32 = 0;
    
    self.consume_until_untyped();
    if self.eof { return 0; }
    
    while 
      Format::is_not_space(self.current_char) && 
      LexerToken::from_char(self.current_char) == LexerToken::Untyped 
    {
      if Format::is_digit(self.current_char) {
        result = result * 10 + (self.current_char as u8 - b'0') as u32;
      }
      self.advance_current_char();
    }

    return result;
  }
  
  fn read_f32(&mut self) -> f32 {
    match self.read_string().unwrap_or_default().parse::<f32>() {
      Ok(result) => return result,
      Err(_) => panic!("{} is not a valid f32", self.current_string)
    }
  }
  
  fn read_f64(&mut self) -> f64 {
    match self.read_string().unwrap_or_default().parse::<f64>() {
      Ok(result) => return result,
      Err(_) => panic!("{} is not a valid f64", self.current_string)
    }
  }
  
  fn read_bool(&mut self) -> bool {
    self.read_string();
    if self.current_string == YES || self.current_string == ONE {
      return true;
    }
    
    if self.current_string == NO {
      return false;
    }
    
    panic!("{} is not a valid bool", self.current_string);
  }
  
  fn read_date(&mut self) -> NaiveDate {
    match chrono::NaiveDate::from_str(
      self.read_string().unwrap_or_default().as_str()
    ) {
      Ok(result) => return result,
      Err(_) => panic!("{} is not a valid date", self.current_string)
    }
  }
  
  fn read_next(&mut self) -> char {
    if let Some(token) = self.pop_next_char() {
      return token;
    }
    
    let buffer_is_consumed = self.buffer_index == self.buffer_size;
    if buffer_is_consumed {
      if !self.eof {
        // Copy next chars to buffer
        self.buffer_size; // = self.reader.Read(buffer, 0, BufferSize);
      }
      
      self.buffer_index = 0;
      
      if self.buffer_size == 0 {
        self.eof = true;
        return NULL_CHAR;
      }
    }
    
    self.buffer_index += 1;
    return self.buffer[self.buffer_index - 1];
  }
  
  fn ensure_left_curly(&mut self) {
    if self.current_token() == LexerToken::Equals {
      self.advance_current_char();
    }
    
    if self.current_token() != LexerToken::LeftCurly {
      panic!("Expected LeftCurly");
    }
  }
  
  fn next_is_bracketed(&mut self) -> bool {
    if let Some(is_bracketed) = self.tag_is_bracketed {
      return is_bracketed;
    }
    
    let mut is_bracketed = false;
    let mut temp_queue = Vec::new();
    
    if self.current_token() != LexerToken::LeftCurly {
      let mut temp_char: char;
      let mut temp_token: LexerToken;
      loop {
        temp_char = self.read_next();
        temp_token = LexerToken::from_char(temp_char);
        
        temp_queue.push(temp_char);
        
        if temp_token == LexerToken::LeftCurly {
          is_bracketed = true;
          break;
        }
        
        if (
          temp_token == LexerToken::Equals || 
          Format::is_space(temp_char)
        ) && !self.eof 
        {
          break;
        }
      }
      
      self.next_chars_empty &= temp_queue.is_empty();
      while !temp_queue.is_empty() {
        self.next_chars.push(temp_queue.remove(0));
      }
    }
    
    self.tag_is_bracketed = Some(is_bracketed);
    return is_bracketed;
  }
  
  fn parse_inner<'a, T: ParadoxReadable>(&mut self, inner_structure: &'a mut T) -> &'a mut T {
    let starting_indent = self.current_indent;
    
    while self.is_in_brackets(starting_indent) {
      if let Some(token) = self.read_string() {
        inner_structure.pdx_token_callback(token);
      }
    }
    
    return inner_structure;
  }
  
  fn read_i32_vec(&self) -> Vec<i32> {
    todo!()
  }
  
  fn read_f64_list(&self) -> Vec<f64> {
    todo!()
  }
  
  fn read_string_list(&self) -> Vec<String> {
    todo!()
  }
  
  fn read_date_list(&self) -> Vec<NaiveDate> {
    todo!()
  }
  
  fn read_inside_brackets(&self, callback: fn(ParadoxParser)) {
    todo!()
  }
  
  fn read_list<T>(&self, read_action: fn() -> T) -> Vec<T> {
    todo!()
  }
  
  fn read_map<TKey, TValue>(
    &self, 
    key_mapping: fn(ParadoxParser) -> TKey,
    value_mapping: fn(ParadoxParser) -> TValue
  ) -> std::collections::HashMap<TKey, TValue> {
    todo!()
  }
  
  fn current_token(&self) -> LexerToken {
    return LexerToken::from_char(self.current_char);
  }
  
  fn next_token(&self) -> Option<LexerToken> {
    todo!()
  }
  
  fn consume_char(&mut self) -> char {
    todo!()
  }
  
  fn consume_token(&mut self) -> LexerToken {
    todo!()
  }
}

impl ParadoxParser {
  pub fn new() -> ParadoxParser {
    return ParadoxParser {
      current_indent: 0,
      next_chars: Vec::new(),
      next_chars_empty: true,
      current_char: NULL_CHAR,
      buffer_index: 0,
      buffer_size: 0,
      buffer: [NULL_CHAR; BufferSize],
      string_buffer: [NULL_CHAR; MaxTokenSize],
      string_buffer_count: 0,
      eof: false,
      tag_is_bracketed: None,
      current_string: String::new()
    }
  }
  
  fn is_in_brackets(&mut self, starting_indent: usize) -> bool {
    if self.current_string.is_empty() {
      self.ensure_left_curly();
    }
    
    loop {
      if self.current_token == LexerToken::RightCurly || 
      self.peek_next_token() == LexerToken::RightCurly
      {
        if self.next_token == Some(LexerToken::RightCurly) {
          self.get_next_token();
        }
        
        while starting_indent != self.current_indent && 
        self.peek_next_token() == LexerToken::RightCurly && 
        !self.eof 
        {
          self.get_next_token();
        }
        
        if self.current_indent == starting_indent {
          break;
        }
      }
      
      if self.string_buffer_count != 0 || self.current_char != NULL_CHAR {
        return true;
      }
      
      if self.eof { break; }
    }
    
    return false;
  }
  
  fn add_to_string_buffer(&mut self, c: char) {
    self.string_buffer_count += 1;
    self.string_buffer[self.string_buffer_count - 1] = c;
  }
  
  fn advance_current_char(&mut self) {
    match self.current_token() {
      LexerToken::LeftCurly => self.current_indent += 1,
      LexerToken::RightCurly => self.current_indent -= 1,
      _ => {}
    };

    self.current_char = self.read_next();
  }
  
  fn consume_string_buffer(&mut self) -> String {
    self.current_string = String::from_iter(
      self.string_buffer
      .into_iter()
      .take(self.string_buffer_count)
    );
    self.string_buffer_count = 0;
    return self.current_string.clone();
  }
  
  fn pop_next_char(&mut self) -> Option<char> {
    if self.next_chars_empty {
      return None;
    } else {
      let result = self.next_chars.remove(0);
      self.next_chars_empty = self.next_chars.is_empty();
      return Some(result);
    }
  } 
  
  fn consume_rest_of_line(&mut self) {
    let mut previous_char = self.current_char; 
    while previous_char != NEWLINE && !self.eof {
      previous_char = self.current_char;
      self.current_char = self.read_next();
    }
  }
  
  fn consume_white_space(&mut self) {
    while Format::is_space(self.current_char) && !self.eof {
      self.current_char = self.read_next();
    }
  }

  fn consume_until_untyped(&mut self) {
    while self.current_token() != LexerToken::Untyped && !self.eof {
      self.current_char = self.read_next();
    }
  }
}
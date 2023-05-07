use crate::paradox_readable::ParadoxReadable;
use chrono::{NaiveDate, TimeZone};
use crate::lexer_token::*;
use crate::extra::*;


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
  fn parse_date(date: String) -> Option<String>;
  fn is_space(c: char) -> bool;
  fn deserialize<T>(data: Vec<u8>) -> T;
  fn parse<'a, T: ParadoxReadable>(data: Vec<u8>, entity: &'a mut T);
  
  // Parse
  fn parse_inner<'a, T: ParadoxReadable>(&mut self, inner_structure: &'a mut T) -> &'a mut T;
  
  // Read Fundamental
  fn read_string(&mut self) -> Option<String>;
  fn read_i32(&mut self) -> i32;
  fn read_i16(&mut self) -> i16;
  fn read_i8(&mut self) -> i8;
  fn read_u32(&mut self) -> u32;
  fn read_u16(&mut self) -> u16;
  fn read_u8(&mut self) -> u8;
  fn read_f32(&mut self) -> f32;
  fn read_f64(&mut self) -> f64;
  fn read_bool(&mut self) -> bool;
  fn read_date(&mut self) -> String;
  
  // Read Advanced
  fn read_i32_vec(&self) -> Vec<i32>;
  fn read_f64_list(&self) -> Vec<f64>;
  fn read_string_list(&self) -> Vec<String>;
  fn read_date_list(&self) -> Vec<String>;
  fn read_inside_brackets(&self, callback: fn(ParadoxParser));
  fn read_list<T>(&self, read_action: fn() -> T) -> Vec<T>;
  fn read_map<TKey, TValue>(
    &self, 
    key_mapping: fn(ParadoxParser) -> TKey,
    value_mapping: fn(ParadoxParser) -> TValue
  ) -> std::collections::HashMap<TKey, TValue>;
  
  // Controls
  fn ensure_left_curly(&mut self);
  fn set_current_token(&mut self, token: LexerToken) -> LexerToken;
  fn get_next_token(&mut self) -> LexerToken;
  fn peek_next_token(&mut self) -> LexerToken;
  fn next_is_bracketed(&mut self) -> bool;
  fn read_next(&mut self) -> char;
}

pub struct ParadoxParser {
  current_indent: usize,
  current_token: LexerToken,
  next_token: Option<LexerToken>,
  next_chars: Vec<char>,
  next_chars_empty: bool,
  current_char: char,
  current_position: usize,
  buffer_size: usize,
  buffer: [char; BufferSize],
  string_buffer: [char; MaxTokenSize],
  string_buffer_count: usize,
  eof: bool,
  tag_is_bracketed: Option<bool>,
  current_string: String  
}

impl IParadoxParser for ParadoxParser {
  fn get_current_indent(&self) -> usize { self.current_indent }
  fn get_current_string(&self) -> String { self.current_string.clone() }
  fn is_end_of_stream(&self) -> bool { self.eof }
  
  fn is_space(c: char) -> bool {
    return c == ' ' || (c >= '\t' && c <= '\r');
  }
  
  fn parse_date(date: String) -> Option<String> {
    let segments: Vec<&str> = date.split('.').collect();
    let mut date = [1, 1, 1, 0, 0, 0];
    
    for i in 0..segments.len() {
      let segment = segments[i];
      let value = segment.parse::<u32>();
      match value {
        Ok(v) => date[i] = v,
        Err(_) => return None
      }
    }
    
    let [y, m, d, hh, mm, s] = date;
    let y_signed: i32 = match y.try_into() {
      Ok(v) => v,
      Err(_) => return None
    };
    
    let first_of_month = NaiveDate::from_ymd_opt(y_signed, m, 1);
    
    let first_of_next_month = match m == 12 {
      true => NaiveDate::from_ymd_opt(y_signed + 1, 1, 1),
      false => NaiveDate::from_ymd_opt(y_signed, m + 1, 1),
      _ => return None
    };
    
    let optional_max_day: Option<u32> = match (first_of_month, first_of_next_month) {
      (Some(start), Some(end)) => end.signed_duration_since(start).num_days(),
      _ => return None
    }.try_into().ok();
    
    let max_day = match optional_max_day {
      Some(d) => d,
      None => return None
    };
    
    let date = match 
    y < 1 || y > 9999 ||
    m < 1 || m > 12 ||
    d < 1 || d > max_day ||
    hh < 0 || hh > 23 ||
    mm < 0 || mm > 59 ||
    s < 0 || s > 59
    {
      true => return None,
      false => NaiveDate::from_ymd_opt(y_signed, m, d)
    };
    
    return date.map(|d| d.to_string());
  }
  
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
    self.current_token = self.get_next_token();
    if self.current_token == LexerToken::Untyped {
      loop {
        self.string_buffer_count += 1;
        self.string_buffer[self.string_buffer_count - 1] = self.current_char;
        
        self.current_char = self.read_next();
        let current_token = LexerToken::from_char(self.current_char);
        if !(
          !ParadoxParser::is_space(self.current_char) &&
          self.set_current_token(current_token) == LexerToken::Untyped && 
          !self.eof
        ) {
          break;
        }
      };
    }
    else if self.current_token == LexerToken::Quote {
      self.current_char = self.read_next();
      while self.current_char != '"' && !self.eof {
        self.string_buffer_count += 1;
        self.string_buffer[self.string_buffer_count - 1] = self.current_char;
        self.current_char = self.read_next();
      }
      
      // Check for partially quoted string of the style "name"_group.
      // If it is, then read string as if untyped.
      let next_char = self.read_next();
      if next_char == '_' {
        self.string_buffer_count += 1;
        self.string_buffer[self.string_buffer_count - 1] = self.current_char;
        self.current_token = self.get_next_token();
        loop {
          self.string_buffer_count += 1;
          self.string_buffer[self.string_buffer_count - 1] = self.current_char;
          self.current_char = self.read_next();
          let current_token = LexerToken::from_char(self.current_char);
          if !(
            !ParadoxParser::is_space(self.current_char) &&
            self.set_current_token(current_token) == LexerToken::Untyped && 
            !self.eof
          ) {
            break;
          }
        };
      } else {
        // Enqueue because it could be important (Equals, quote, etc.)
        self.next_chars.push(next_char);
        self.next_chars_empty = false;
      }
    } else if 
    self.current_token == LexerToken::LeftCurly &&
    self.peek_next_token() == LexerToken::RightCurly {
      return None;
    } else {
      return self.read_string();
    }
    
    self.current_string = String::from_iter(
      self.string_buffer
      .into_iter()
      .take(self.string_buffer_count)
    );
    self.string_buffer_count = 0;
    return Some(self.current_string.clone());
  }
  
  fn read_i32(&mut self) -> i32 {
    let mut result: i32 = 0;
    let mut sign = 1;
    
    while self.get_next_token() != LexerToken::Untyped && ! self.eof {};
    
    if self.eof {
      return 0;
    }
    
    loop {
      if self.current_char >= '0' && self.current_char <= '9' {
        result = result * 10 + (self.current_char as u8 - b'0') as i32;
      } else if self.current_char == '-' {
        sign = -1;
      }
      
      self.current_char = self.read_next();
      let current_token = LexerToken::from_char(self.current_char);
      if !(
        ParadoxParser::is_space(self.current_char) &&
        self.set_current_token(current_token) == LexerToken::Untyped &&
        !self.eof
      ) {
        break;
      }
    }
    
    return sign * result;
  }
  
  fn read_i16(&mut self) -> i16 {
    return self.read_i32() as i16;
  }
  
  fn read_i8(&mut self) -> i8 {
    return self.read_i32() as i8;
  }
  
  fn read_u32(&mut self) -> u32 {
    let mut result: u32 = 0;
    
    while self.get_next_token() != LexerToken::Untyped && !self.eof {};
    
    if self.eof {
      return 0;
    }
    
    loop {
      result = (10 * result) + (self.current_char as u8 - b'0') as u32;
      
      self.current_char = self.read_next();
      let current_token = LexerToken::from_char(self.current_char);
      if !(
        ParadoxParser::is_space(self.current_char) &&
        self.set_current_token(current_token) == LexerToken::Untyped &&
        !self.eof
      ) {
        break;
      }
    }
    
    return result;
  }
  
  fn read_u16(&mut self) -> u16 {
    return self.read_u32() as u16;
  }
  
  fn read_u8(&mut self) -> u8 {
    return self.read_u32() as u8;
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
    if self.current_string == "yes" || self.current_string == "1" {
      return true;
    }
    
    if self.current_string == "no" {
      return false;
    }
    
    panic!("{} is not a valid bool", self.current_string);
  }
  
  fn read_date(&mut self) -> String {
    match chrono::Utc.datetime_from_str(
      self.read_string().unwrap_or_default().as_str(), "%Y.%m.%d"
    ) {
      Ok(result) => return result.to_string(),
      Err(_) => panic!("{} is not a valid date", self.current_string)
    }
  }
  
  fn set_current_token(&mut self, token: LexerToken) -> LexerToken{
    self.current_indent = match token {
      LexerToken::LeftCurly => self.current_indent + 1,
      LexerToken::RightCurly if self.current_indent > 0 => self.current_indent - 1,
      _ => 0
    };
    self.current_token = token;
    return token;
  }
  
  fn read_next(&mut self) -> char {
    if !self.next_chars_empty {
      let result = self.next_chars.remove(0);
      self.next_chars_empty = self.next_chars.is_empty();
      return result;
    }
    
    if self.current_position == self.buffer_size {
      if !self.eof {
        self.buffer_size; // = self.reader.Read(buffer, 0, BufferSize);
      }
      
      self.current_position = 0;
      
      if self.buffer_size == 0 {
        self.eof = true;
        return '\0';
      }
    }
    
    self.current_position += 1;
    return self.buffer[self.current_position - 1];
  }
  
  fn get_next_token(&mut self) -> LexerToken {
    self.tag_is_bracketed = None;
    if let Some(token) = self.next_token {
      self.next_token = None;
      return self.set_current_token(token);
    };
    
    if self.current_char == POUND {
      while self.current_char != '\n' && !self.eof {
        self.current_char = self.read_next();
        self.set_current_token(LexerToken::from_char(self.current_char));
      }
    }
    
    while ParadoxParser::is_space(self.current_char) && !self.eof {
      self.current_char = self.read_next();
    }
    
    let current_token = LexerToken::from_char(self.current_char);
    self.set_current_token(current_token);
    if current_token == LexerToken::Comment {
      while self.current_char != '\n' && !self.eof {
        self.current_char = self.read_next();
      }
      return self.get_next_token();
    }
    
    return self.current_token;
  }
  
  fn peek_next_token(&mut self) -> LexerToken {
    if let Some(token) = self.next_token {
      self.set_current_token(token);
    }
    
    while ParadoxParser::is_space(self.current_char) && !self.eof {
      self.current_char = self.read_next();
    }
    
    let next_token = LexerToken::from_char(self.current_char);
    self.next_token = Some(next_token);
    if next_token == LexerToken::Comment {
      while self.current_char != '\n' && !self.eof {
        self.current_char = self.read_next();
      }
      return self.peek_next_token();
    }
    
    return self.next_token.unwrap();
  }
  
  fn ensure_left_curly(&mut self) {
    if self.current_token == LexerToken::Equals {
      self.current_token = self.get_next_token();
    }
    
    if self.current_token != LexerToken::LeftCurly {
      panic!("Expected LeftCurly");
    }
  }
  
  fn next_is_bracketed(&mut self) -> bool {
    if let Some(is_bracketed) = self.tag_is_bracketed {
      return is_bracketed;
    }
    
    let mut is_bracketed = false;
    let mut temp_queue = Vec::new();
    
    if self.current_token != LexerToken::LeftCurly {
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
        
        if (temp_token == LexerToken::Equals || ParadoxParser::is_space(temp_char)) && !self.eof {
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
  
  fn read_date_list(&self) -> Vec<String> {
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
}

impl ParadoxParser {
  pub fn new() -> ParadoxParser {
    return ParadoxParser {
      current_indent: 0,
      current_token: LexerToken::Untyped,
      next_token: None,
      next_chars: Vec::new(),
      next_chars_empty: true,
      current_char: '\0',
      current_position: 0,
      buffer_size: 0,
      buffer: ['\0'; BufferSize],
      string_buffer: ['\0'; MaxTokenSize],
      string_buffer_count: 0,
      eof: false,
      tag_is_bracketed: None,
      current_string: String::new()
    }
  }
  
  pub fn is_in_brackets(&mut self, starting_indent: usize) -> bool {
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
      
      if self.string_buffer_count != 0 || self.current_char != '\0' {
        return true;
      }
      
      if self.eof { break; }
    }

    return false;
  }
}
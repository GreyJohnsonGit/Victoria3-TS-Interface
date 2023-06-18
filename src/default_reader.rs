use jomini::{
  text::ValueReader, 
  Windows1252Encoding
};

/// Default reader type for PDX files.
pub type DefaultReader<'a, 'b> = ValueReader<'a, 'b, Windows1252Encoding>;
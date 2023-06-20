use jomini::{
  text::{ValueReader, ObjectReader}, 
  Windows1252Encoding
};

/// Default reader for value types for PDX files.
pub type DefaultValueReader<'a, 'b> = ValueReader<'a, 'b, Windows1252Encoding>;

/// Default reader for object types for PDX files.
pub type DefaultObjectReader<'a, 'b> = ObjectReader<'a, 'b, Windows1252Encoding>;
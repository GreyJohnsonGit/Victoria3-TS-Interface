use jomini::TextTape;
use crate::{pdx_builder::IPdxBuilder, default_reader::DefaultObjectReader};

/// Parser to convert PDX Strings to internal structs.
pub trait IPdxParser<Product> {

  /// Deserialize the given `text` to a Vec of internal structs.
  fn parse(&self, text: &str) -> Result<Vec<Product>, ()>;

  /// Deserialize the given `reader` to a Vec of internal structs.
  fn parse_reader(&self, reader: &DefaultObjectReader) -> Result<Vec<Product>, ()>;

  /// Deserialize a single object with from the given `reader` with the given
  /// `root` key.
  fn parse_object(&self, 
    root: &str, 
    reader: &DefaultObjectReader
  ) -> Result<Product, ()>;

  /// Create a new instance of the parser.
  fn create_new(&self) -> Box<dyn IPdxParser<Product>>;
}

pub struct PdxParser<Product> {
  builder_template: Box<dyn IPdxBuilder<Product>>
}

impl <Product: 'static> PdxParser<Product> {
  pub fn new( 
    builder_template: &Box<dyn IPdxBuilder<Product>>
  ) -> PdxParser<Product> {
    PdxParser { 
      builder_template: builder_template.create_new()
    }
  }

  pub fn new_boxed(
    builder_template: &Box<dyn IPdxBuilder<Product>>
  ) -> Box<dyn IPdxParser<Product>> {
    Box::new(Self::new(builder_template))
  }
}

impl <Product: 'static> IPdxParser<Product> for PdxParser<Product> {
  fn parse(&self, text: &str) -> Result<Vec<Product>, ()> {
    let text = text.as_bytes();
    
    let tape = match TextTape::from_slice(text) {
      Ok(t) => t,
      Err(_) => return Err(()),
    };
    
    let reader = tape.windows1252_reader();
    
    self.parse_reader(&reader)
  }

  fn parse_reader(&self, reader: &DefaultObjectReader) -> Result<Vec<Product>, ()> {
    let mut products: Vec<Product> = vec![];
    
    for (root, _, inner) in reader.fields() {
      let definition = match inner.read_object() {
        Ok(d) => d,
        Err(_) => return Err(()),
      };
      
      let product = self.parse_object(&root.read_string(), &definition);
      
      match product {
        Ok(p) => products.push(p),
        Err(_) => return Err(()),
      }
    }
    
    return Ok(products);
  }

  fn parse_object(&self, 
    root: &str,
    reader: &DefaultObjectReader 
  ) -> Result<Product, ()>{
    let mut builder = self.builder_template.create_new();
    builder.apply_root(&root);
    
    for (key, _, value) in reader.fields() {
      let _ = builder.apply(&key.read_string(), &value);
    }
    
    builder.build()
  }

  fn create_new(&self) -> Box<dyn IPdxParser<Product>> {
    Box::new(Self::new(&self.builder_template))
  }
}
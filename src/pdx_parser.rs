use jomini::TextTape;
use crate::pdx_builder::IPdxBuilder;

/// Parser to convert PDX Strings to internal structs.
pub trait IPdxParser<Product> {

  /// Deserialize the given `text` to a Vec of internal structs.
  fn parse(&self, text: &str) -> Result<Vec<Product>, ()>;
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

impl <Product> IPdxParser<Product> for PdxParser<Product> {
  fn parse(&self, text: &str) -> Result<Vec<Product>, ()> {
    let text = text.as_bytes();
    
    let tape = match TextTape::from_slice(text) {
      Ok(t) => t,
      Err(_) => return Err(()),
    };
    
    let reader = tape.windows1252_reader();
    
    let mut products: Vec<Product> = vec![];
    
    for (root, _, inner) in reader.fields() {
      let mut builder = self.builder_template.create_new();
      builder.apply_root(&root.read_string());
      
      let definition = match inner.read_object() {
        Ok(d) => d,
        Err(_) => return Err(()),
      };
      
      for (key, _, value) in definition.fields() {
        let _ = builder.apply(&key.read_string(), &value);
      }
      
      match builder.build() {
        Ok(b) => products.push(b),
        Err(_) => return Err(()),
      }
    }
    
    return Ok(products);
  }
}
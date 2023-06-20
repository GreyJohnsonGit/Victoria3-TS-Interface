use crate::default_reader::DefaultValueReader;

/// Describes structs that can be used to build internal structs by applying
/// parsed key (i.e. token) and value pairs.
/// 
/// @notes This trait also include the `create_new` function to implement the
/// template design pattern.
pub trait IPdxBuilder<Product> {

  /// Apply the initial token which often indicates the identity of the given
  /// PDX statement.
  fn apply_root(&mut self, root: &str);
  
  /// Interpret a given key value pair in the context of the `Product`. Fails
  /// when the `value` does not 
  fn apply(&mut self, token: &str, value: &DefaultValueReader) -> Result<(), ()>;
  
  /// Finalizing struct creation. Ok will return finalized object. Err will
  /// log missing or invalid fields.
  fn build(self: Box<Self>) -> Result<Product, ()>;

  /// Template design pattern implementation. Creates a clone of builder with
  /// the same internal state.
  fn create_new(&self) -> Box<dyn IPdxBuilder<Product>>;
}
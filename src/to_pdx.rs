/// Trait to describe structs that can be serialized to a PDX String.
pub trait IToPdx {
  /// Serializes struct to PDX String.
  fn to_pdx(&self) -> String;
}
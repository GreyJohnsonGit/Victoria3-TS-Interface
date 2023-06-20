#[macro_export]
macro_rules! declare_get_and_set {
  ($property:ident, $setter:ident, $property_type:ty) => {
    fn $property(&self) -> $property_type;
    fn $setter(&mut self, $property: $property_type);
  };
}

#[macro_export]
macro_rules! define_get_and_set {
  ($property:ident, $setter:ident, $property_type:ty) => {
    fn $property(&self) -> $property_type { self.$property.clone() }
    fn $setter(&mut self, $property: $property_type) { 
      let _ = std::mem::replace(&mut self.$property, $property);
    }
  };
}

#[macro_export]
macro_rules! define_applier {
    ($method:ident, $read_method:ident, $property:ident, $property_name:tt, $property_type:tt) => {
      fn $method(&mut self, value: &DefaultValueReader) -> Result<(), ()> {
        value.$read_method()
          .map(|value| self.$property = Some(value))
          .map_err(|_| { let _ = self.logger.coerce_error($property_name, $property_type); })
      }
    };
}
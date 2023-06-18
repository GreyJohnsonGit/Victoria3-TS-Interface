use crate::logger::ILogger;

impl dyn ILogger {
  pub fn coerce_error(&self, from: &str, to: &str) -> Result<(), ()> {
    Err(self.error(&format!(
      "Failed to coerce `{}` to `{}`", from, to
    )))
  }

  pub fn missing_property_error(&self, 
    object_type: &str, 
    property: &str
  ) -> Result<(), ()> {
    Err(self.error(&format!(
      "`{}` is missing property `{}`", object_type, property
    )))
  }
}
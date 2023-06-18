use crate::logger::ILogger;
use std::fmt::Display;

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

  pub fn no_entity_found(&self, entity: &str, path: impl Display) -> Result<(), ()> {
    Ok(self.warning(&format!(
      "No `{}` found in {}", entity, path
    )))
  }

  pub fn failed_to_parse(&self, path: impl Display) -> Result<(), ()> {
    Ok(self.warning(&format!(
      "Failed to parse `{}`", path
    )))
  }
}
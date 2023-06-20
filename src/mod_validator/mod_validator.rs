use std::collections::HashSet;
use crate::{mod_state::IModState, logger::{ILogger, LogLevel}};

/// Describes structs that can validate a mod.
/// 
/// @notes This trait aims to catch common errors. While catching all errors in
/// advance would be great, we cannot ensure that.
pub trait IModValidator {
  /// Check that all references to cultures use a culture that is defined.
  fn cultures_are_defined(&self, mod_state: &Box<dyn IModState>) -> Result<(), ()>;

  /// Check that no country tag is used twice.
  fn no_duplicate_tags(&self, mod_state: &Box<dyn IModState>) -> Result<(), ()>;
}

pub struct ModValidator {
  logger: Box<dyn ILogger>
}

impl ModValidator {
  pub fn new(logger: &Box<dyn ILogger>) -> ModValidator { 
    ModValidator { logger: logger.create_new() } 
  }

  pub fn new_boxed(logger: &Box<dyn ILogger>) -> Box<dyn IModValidator> {
    Box::new(ModValidator::new(logger))
  }
}

impl IModValidator for ModValidator {
  fn cultures_are_defined(&self, 
    mod_state: &Box<dyn IModState>
  ) -> Result<(), ()> {
    let mut error_flag = false;
    let culture_files = mod_state.get_culture_files();
    let country_definition_files = mod_state.get_country_definition_files();
    
    let mut culture_string_id_lookup = HashSet::new();
    for (_, cultures) in culture_files {
      for culture in cultures {
        culture_string_id_lookup.insert(culture.string_id());
      }
    }
    
    for (file_name, country_definitions) in country_definition_files {
      for definition in country_definitions {
        for culture in definition.cultures() {
          if !culture_string_id_lookup.contains(&culture) {
            error_flag = true;
            self.logger.log(LogLevel::Error, &format!(
              "Culture `{}` is Undefined! @ {}", culture, file_name
            ));
          }
        }
      }
    }

    match error_flag {
      true => Err(()),
      false => Ok(())
    }
  }
  
  fn no_duplicate_tags(&self,
    mod_state: &Box<dyn IModState>
  ) -> Result<(), ()> {
    let mut error_flag = false;
    let country_definition_files = mod_state.get_country_definition_files();

    let mut tag_lookup = HashSet::new();
    for (file_name, definitions) in country_definition_files {
      for definition in definitions {
        let tag = definition.tag();
        if tag_lookup.contains(&tag) {
          error_flag = true;
          self.logger.log(LogLevel::Error, &format!(
            "Duplicate Tag `{}` @ {}", tag, file_name
          ));
        } else {
          tag_lookup.insert(tag);
        }
      }
    }

    match error_flag {
      true => Err(()),
      false => Ok(())
    }
  }
}
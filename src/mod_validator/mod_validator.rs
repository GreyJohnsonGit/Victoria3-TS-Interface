use std::collections::HashSet;
use crate::mod_state::IModState;

pub trait IModValidator {
  fn cultures_are_defined(&self,
    mod_state: &Box<dyn IModState>
  ) -> Result<(), Vec<String>>;
  
  fn no_duplicate_tags(&self,
    mod_state: &Box<dyn IModState>
  ) -> Result<(), Vec<String>>;
}

pub struct ModValidator {}

impl ModValidator {
  pub fn new() -> ModValidator { ModValidator {} }
}

impl IModValidator for ModValidator {
  fn cultures_are_defined(&self, 
    mod_state: &Box<dyn IModState>
  ) -> Result<(), Vec<String>> {
    let mut errors = Vec::<String>::new();
    
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
            errors.push(format!("Culture `{}` is Undefined! @ {}", culture, file_name))
          }
        }
      }
    }
    
    match errors.len() {
      0 => Ok(()),
      _ => Err(errors)
    }
  }
  
  fn no_duplicate_tags(&self,
    mod_state: &Box<dyn IModState>
  ) -> Result<(), Vec<String>> {
    let mut errors = Vec::<String>::new();

    let country_definition_files = mod_state.get_country_definition_files();

    let mut tag_lookup = HashSet::new();
    for (file_name, definitions) in country_definition_files {
      for definition in definitions {
        let tag = definition.tag();
        if tag_lookup.contains(&tag) {
          errors.push(format!("Duplicate Tag `{}`! @ {}", tag, file_name));
        } else {
          tag_lookup.insert(tag);
        }
      }
    }

    match errors.len() {
      0 => Ok(()),
      _ => Err(errors)
    }
  }
}
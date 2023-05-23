use std::{path::Path, fs, ops::Not};
use crate::{config::IConfig, mod_state::IModState};

pub trait IModBuilder {
  fn validate(&self) -> Result<bool, String>;
  fn save(&self) -> Result<bool, String>;
}

pub struct ModBuilder {
  config: Box<dyn IConfig>,
  mod_state: Box<dyn IModState>
}

impl ModBuilder {
  pub fn new(config: Box<dyn IConfig>, mod_state: Box<dyn IModState>) -> ModBuilder {
    ModBuilder { config, mod_state }
  }
}

impl IModBuilder for ModBuilder {
  fn validate(&self) -> Result<bool, String> {
    println!("Validating mod files...");    
    return Ok(true);
  }
  
  fn save(&self) -> Result<bool, String> {
    let mod_path_string = self.config.get_mod_path();
    let mod_path = Path::new(&mod_path_string);

    let dir_check_result = mod_path
      .is_dir()
      .not()
      .then(|| { fs::create_dir_all(&mod_path)});

    if let Some(Err(error)) = dir_check_result {
      return Err(format!("Mod path {} does not exist and could not be created! Error: {}", mod_path.display(), error));
    }

    println!("Saving `Country Definition` files"); {
      let country_definition_path = mod_path.join("common\\country_definitions");
      fs::create_dir_all(&country_definition_path).ok();
      for (file_name, definitions) in self.mod_state.get_country_definitions() {
        let file_path = country_definition_path.join(file_name);
        let contents = definitions
          .iter()
          .map(|definition| definition.as_pdx())
          .collect::<Vec<String>>()
          .join("\n\n");
        fs::write(file_path, contents).ok();
      }
    }

    println!("Saving `Culture` files"); {
      let culture_path = mod_path.join("common\\cultures");
      fs::create_dir_all(&culture_path).ok();
      for (file_name, cultures) in self.mod_state.get_cultures() {
        let file_path = culture_path.join(file_name);
        let contents = cultures
          .iter()
          .map(|culture| culture.as_pdx())
          .collect::<Vec<String>>()
          .join("\n\n");
        fs::write(file_path, contents).ok();
      }
    }

    println!("Saving mod files to {}...", mod_path_string);
    return Ok(true);
  }
}
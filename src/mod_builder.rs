use std::{path::Path, fs, ops::Not};
use crate::{config::IConfig, mod_state::IModState, mod_validator::mod_validator::IModValidator, logger::{ILogger, LogLevel}, to_pdx::IToPdx};

pub trait IModBuilder {
  fn validate_with(&self, validator: Box<dyn IModValidator>) -> Result<(), ()>;
  fn save(&self) -> Result<bool, String>;
}

pub struct ModBuilder {
  logger: Box<dyn ILogger>,
  config: Box<dyn IConfig>,
  mod_state: Box<dyn IModState>
}

impl ModBuilder {
  pub fn new(
    logger: Box<dyn ILogger>,
    config: Box<dyn IConfig>,
    mod_state: Box<dyn IModState>,
  ) -> ModBuilder {
    ModBuilder { 
      config, 
      mod_state,
      logger
    }
  }
}

impl IModBuilder for ModBuilder {
  fn validate_with(&self, validator: Box<dyn IModValidator>) -> Result<(), ()> {
    self.logger.log_str(LogLevel::Info, "Validating mod files...");
    
    validator.cultures_are_defined(&self.mod_state)
      .and(validator.no_duplicate_tags(&self.mod_state))
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

    self.logger.log(LogLevel::Info, &format!(
      "Saving Mod @ {}", mod_path.display()
    ));

    {
      let country_definition_path = mod_path.join("common\\country_definitions");
      fs::create_dir_all(&country_definition_path).ok();
      for (file_name, definitions) in self.mod_state.get_country_definition_files() {
        let file_path = country_definition_path.join(file_name);
        let contents = definitions
          .iter()
          .map(|definition| definition.to_pdx())
          .collect::<Vec<String>>()
          .join("\n\n");
        fs::write(file_path, contents).ok();
      }
    }

    {
      let culture_path = mod_path.join("common\\cultures");
      fs::create_dir_all(&culture_path).ok();
      for (file_name, cultures) in self.mod_state.get_culture_files() {
        let file_path = culture_path.join(file_name);
        let contents = cultures
          .iter()
          .map(|culture| culture.to_pdx())
          .collect::<Vec<String>>()
          .join("\n\n");
        fs::write(file_path, contents).ok();
      }
    }

    return Ok(true);
  }
}
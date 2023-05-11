use std::{path::Path, fs, ops::Not};
use crate::config::IConfig;

pub trait IModBuilder {
  fn validate(&self) -> Result<bool, String>;
  fn save(&self) -> Result<bool, String>;
}

pub struct ModBuilder {
  config: Box<dyn IConfig>,
}

impl ModBuilder {
  pub fn new(config: Box<dyn IConfig>) -> ModBuilder {
    ModBuilder { config }
  }
}

impl IModBuilder for ModBuilder {
  fn validate(&self) -> Result<bool, String> {
    println!("Validating mod files...");    
    return Ok(true);
  }
  
  fn save(&self) -> Result<bool, String> {
    let path_string = self.config.get_mod_path();
    let mod_path = Path::new(&path_string);

    let dir_check_result = mod_path
      .is_dir()
      .not()
      .then(|| { fs::create_dir_all(&mod_path)});

    if let Some(Err(error)) = dir_check_result {
      return Err(format!("Mod path {} does not exist and could not be created! Error: {}", mod_path.display(), error));
    }

    println!("Saving mod files to {}...", path_string);
    return Ok(true);
  }
}
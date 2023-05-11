use crate::config::IConfig;
use crate::mod_builder::{IModBuilder, ModBuilder};
use std::fs;
use std::path::Path;
use crate::country_definition::country_definition::CountryDefinition;

pub trait IFileLoader {
  fn load_vanilla(&self) -> Result<bool, String>;
  fn load_pdx(&self) -> Result<bool, String>;
  fn load_json(&self) -> Result<bool, String>;
  fn create_mod_builder(&self) -> Box<dyn IModBuilder>;
}

pub struct FileLoader {
  config: Box<dyn IConfig>,
}

impl FileLoader {
  pub fn new(config: Box<dyn IConfig>) -> FileLoader {
    return FileLoader { config }
  }
}

impl IFileLoader for FileLoader {
  fn load_vanilla(&self) -> Result<bool, String> {
    let path_string = self.config.get_vanilla_path();
    let path = Path::new(&path_string);
    
    if path.is_dir() == false {
      return Err(format!("Vanilla path {} does not exist!", path.display()));
    }

    { let path_extension = "common/country_definitions";
      let path = path.join(path_extension);
      let files = match fs::read_dir(path.clone()) {
        Err(e) => return Err(format!("{} @ {}", e, path.to_str().unwrap())),
        Ok(files) => files,
      };

      for file in files {
        if let Ok(file) = file {
          let text = fs::read_to_string(file.path()).unwrap();
          let country_definitions = CountryDefinition::from_pdx(text);
        }
      }
    };
      
    
    println!("Loading vanilla files from {}...", path.display());
    
    return Ok(true);
  }
  
  fn load_pdx(&self) -> Result<bool, String> {
    println!("Loading pdx files from {}...", self.config.get_pdx_path());    
    return Ok(true);
  }
  
  fn load_json(&self) -> Result<bool, String> {
    println!("Loading json files from {}...", self.config.get_json_path());    
    return Ok(true);
  }
  
  fn create_mod_builder(&self) -> Box<dyn IModBuilder> {
    return Box::from(ModBuilder::new(self.config.clone_box()));
  }
}

#[cfg(test)]
mod tests {
  use crate::config::MockIConfig;
  use super::*;

  #[test]
  fn load_vanilla_with_valid_path_ok() {
    // Arrange
    let temp_directory = tempfile::tempdir().unwrap();
    let path: String = temp_directory.path().to_string_lossy().into();
    fs::create_dir_all(path.clone() + "/common/country_definitions/").unwrap();

    let mut config = MockIConfig::new();
    config.expect_get_vanilla_path()
      .returning(move || path.clone());

    let loader = FileLoader::new(
      Box::from(config)
    );

    // Act
    let result = loader.load_vanilla();

    // Assert
    println!("{:?}", result);
    assert!(result.is_ok());
  }

  #[test]
  fn load_vanilla_with_invalid_path_err() {
    // Arrange
    let temp_directory = tempfile::tempdir().unwrap();
    let path: String = temp_directory
      .path().join("does_not_exist")
      .to_string_lossy().into();

    let mut config = MockIConfig::new();
    config.expect_get_vanilla_path()
      .returning(move || path.clone());

    let loader = FileLoader::new(
      Box::from(config)
    );

    // Act
    let result = loader.load_vanilla();

    // Assert
    assert!(result.is_err());
  }
}
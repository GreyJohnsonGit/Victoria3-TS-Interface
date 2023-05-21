use crate::config::IConfig;
use crate::country_definition::country_definition_factory::ICountryDefinitionFactory;
use crate::mod_builder::ModBuilder;
use crate::mod_state::{IModState, ModState};
use std::fs;
use std::path::Path;
use crate::country_definition::country_definition::CountryDefinition;

pub trait IFileLoader {
  fn load_vanilla(&mut self) -> Result<bool, String>;
  fn load_pdx(&self) -> Result<bool, String>;
  fn load_json(&self) -> Result<bool, String>;
  fn create_mod_builder(self) -> Box<ModBuilder>;
}

pub struct FileLoader<'a> {
  config: &'a Box<dyn IConfig>,
  country_definition_factory: &'a Box<dyn ICountryDefinitionFactory>,
  mod_state: Option<Box<dyn IModState>>
}

impl FileLoader<'_> {
  pub fn new<'a>(
    config: &'a Box<dyn IConfig>,
    country_definition_factory: &'a Box<dyn ICountryDefinitionFactory>,
  ) -> FileLoader<'a> {
    let mod_state: Box<dyn IModState> = Box::from(ModState::new());
    return FileLoader { 
      config, 
      country_definition_factory,
      mod_state: Some(mod_state)
    }
  }
}

impl IFileLoader for FileLoader<'_> {
  fn load_vanilla(&mut self) -> Result<bool, String> {
    let path_string = self.config.get_vanilla_path();
    let path = Path::new(&path_string);
    
    if path.is_dir() == false {
      return Err(format!("Vanilla path {} does not exist!", path.display()));
    }

    { let path_extension = "common\\country_definitions";
      let path = path.join(path_extension);
      let directory = match fs::read_dir(path.clone()) {
        Err(e) => return Err(format!("{} @ {}", e, path.to_str().unwrap())),
        Ok(files) => files,
      };

      for entry in directory {
        if let Ok(entry) = entry {
          let file_path = entry.path();
          let file_path_str = file_path.to_str().unwrap_or("No Path");
          let file_text = fs::read_to_string(file_path.clone()).unwrap();
          
          let definitions = CountryDefinition::from_pdx(
            file_text, 
            &self.country_definition_factory
          );

          let definitions = match definitions {
            Err(e) => {
              println!("{} @ {}", e, file_path_str);
              continue;
            },
            Ok(d) => d,
          };

          self.mod_state.as_mut().map(|state| {
            let file_name = Path::file_name(&file_path)
              .map(|file_name| file_name.to_str())
              .flatten()
              .unwrap_or("no_file_name.txt");
            state.add_file(
              String::from(file_name), 
              definitions
            )
          });
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
  
  fn create_mod_builder(mut self) -> Box<ModBuilder> {
    let mod_state = std::mem::take(&mut self.mod_state);
    let config = self.config.clone_box();
    let mod_builder = ModBuilder::new(config, mod_state.unwrap());

    Box::from(mod_builder)
  }
}

#[cfg(test)]
mod tests {
  use crate::{config::MockIConfig, country_definition::country_definition_factory::CountryDefinitionFactory};
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

    let config: Box<dyn IConfig> = Box::from(config);
    let factory = CountryDefinitionFactory::new_boxed();
    let mut loader = FileLoader::new(
      &config,
      &factory
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
  
      let config: Box<dyn IConfig> = Box::from(config);
      let factory = CountryDefinitionFactory::new_boxed();
      let mut loader = FileLoader::new(
        &config,
        &factory
      );

    // Act
    let result = loader.load_vanilla();

    // Assert
    assert!(result.is_err());
  }
}
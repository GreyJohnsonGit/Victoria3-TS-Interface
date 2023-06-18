use crate::config::IConfig;
use crate::country_definition::country_definition::ICountryDefinition;
use crate::culture::culture::Culture;
use crate::logger::{ILogger, LogLevel};
use crate::mod_builder::ModBuilder;
use crate::mod_state::{IModState, ModState};
use crate::pdx_parser::IPdxParser;
use std::fs;
use std::path::Path;

/// Describes structs that can load files from disk and use them to create a
/// IModeBuilder.
pub trait IFileLoader {

  /// Loads vanilla files from the game directory.
  fn load_vanilla(&mut self) -> Result<(), ()>;

  /// Loads PDX files from the pdx directory.
  fn load_pdx(&mut self) -> Result<(), ()>;

  /// Loads JSON files from the json directory.
  fn load_json(&self) -> Result<(), ()>;

  /// Parses files into internal structs and returns a ModBuilder.
  fn create_mod_builder(self) -> Box<ModBuilder>;
}

pub struct FileLoader<'a> {
  config: &'a Box<dyn IConfig>,
  mod_state: Option<Box<dyn IModState>>,
  parser: &'a Box<dyn IPdxParser<Box<dyn ICountryDefinition>>>,
  logger: Box<dyn ILogger>,
}

impl FileLoader<'_> {
  pub fn new<'a>(
    config: &'a Box<dyn IConfig>,
    parser: &'a Box<dyn IPdxParser<Box<dyn ICountryDefinition>>>,
    logger: &'a Box<dyn ILogger>
  ) -> FileLoader<'a> {
    let mod_state: Box<dyn IModState> = Box::from(ModState::new());
    return FileLoader { 
      config,
      mod_state: Some(mod_state),
      parser,
      logger: logger.clone_boxed()
    }
  }
  
  fn load_country_definitions(&mut self, path_string: String) -> Result<(), ()> {
    let path = Path::new(&path_string).join("common\\country_definitions");
    let directory = match fs::read_dir(path.clone()) {
      Err(_) => {
        self.logger.log(LogLevel::Warning, 
          &format!("No Country Definitions found in {}", path.display())
        );
        return Ok(());
      },
      Ok(files) => files,
    };
    
    for entry in directory {
      if let Ok(entry) = entry {
        let file_path = entry.path();
        let file_text = fs::read_to_string(file_path.clone())
          .unwrap_or(String::new());
        
        let definitions = self.parser.parse(&file_text);
        
        let definitions = match definitions {
          Err(_) => {
            self.logger.log(LogLevel::Warning, 
              &format!("Failed to parse `{}`", path.display())
            );
            continue;
          },
          Ok(d) => d,
        };
        
        self.mod_state.as_mut().map(|state| {
          let file_name = Path::file_name(&file_path)
            .map(|file_name| file_name.to_str())
            .flatten()
            .unwrap_or("no_file_name.txt");
          state.set_country_definitions_file(
            file_name.to_string(), 
            definitions
          )
        });
      }
    };
    
    return Ok(());
  }
  
  fn load_cultures(&mut self, path_string: String) -> Result<(), ()> {
    let path = Path::new(&path_string).join("common\\cultures");
    
    let directory = match fs::read_dir(path.clone()) {
      Err(_) => {
        self.logger.log(LogLevel::Warning, 
          &format!("No Cultures found in {}", path.display())
        );
        return Ok(());
      },
      Ok(files) => files,
    };
    
    for entry in directory {
      if let Ok(entry) = entry {
        let file_path = entry.path();
        let file_text = fs::read_to_string(file_path.clone())
        .unwrap_or(String::new());
        
        let cultures = Culture::from_pdx(
          file_text, 
        );
        
        let cultures = match cultures {
          Err(e) => {
            self.logger.log(LogLevel::Warning, 
              &format!("{} @ {}", e, path.display())
            );
            continue;
          },
          Ok(d) => d,
        };
        
        self.mod_state.as_mut().map(|state| {
          let file_name = Path::file_name(&file_path)
            .map(|file_name| file_name.to_str())
            .flatten()
            .unwrap_or("no_file_name.txt");
          state.set_cultures_file(
            file_name.to_string(), 
            cultures
          )
        });
      }
    };
    
    return Ok(());
  }
}

impl IFileLoader for FileLoader<'_> {
  fn load_vanilla(&mut self) -> Result<(), ()> {
    let path_string = self.config.get_vanilla_path();
    let path = Path::new(&path_string);
    
    self.logger.log(LogLevel::Info, 
      &format!("Loading Vanilla Files @ {}", path.display())
    );
    
    if path.is_dir() == false {
      self.logger.log(LogLevel::Error, 
        &format!("Vanilla Path Does Not Exist @ {}", path.display())
      );
      return Err(());
    }
    
    self.load_country_definitions(path_string.clone())
      .and(self.load_cultures(path_string.clone()))
  }
  
  fn load_pdx(&mut self) -> Result<(), ()> {
    let path_string = self.config.get_pdx_path();
    let path = Path::new(&path_string);
    
    self.logger.log(LogLevel::Info, 
      &format!("Loading PDX Files @ {}", path.display())
    );
    
    if path.is_dir() == false {
      self.logger.log(LogLevel::Error, 
        &format!("PDX Path Does Not Exist @ {}", path.display())
      );
      return Err(());
    }

    return Ok(())
      .and(self.load_country_definitions(path_string.clone()))
      .and(self.load_cultures(path_string.clone()))
  }
  
  fn load_json(&self) -> Result<(), ()> {
    return Ok(());
  }
  
  fn create_mod_builder(mut self) -> Box<ModBuilder> {
    let mod_state = std::mem::take(&mut self.mod_state);
    let config = self.config.clone_box();
    let mod_builder = ModBuilder::new(
      self.logger.clone_boxed(),
      config, 
      mod_state.unwrap()
    );
    
    Box::from(mod_builder)
  }
}
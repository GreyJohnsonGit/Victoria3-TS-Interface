use crate::config::IConfig;
use crate::logger::{ILogger, LogLevel};
use crate::mod_builder::ModBuilder;
use crate::mod_state::{IModState, ModState};
use crate::parser_factory::IParserFactory;
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
  parser_factory: Box<dyn IParserFactory>,
  logger: Box<dyn ILogger>,
}

impl FileLoader<'_> {
  pub fn new<'a>(
    config: &'a Box<dyn IConfig>,
    parser_factory: Box<dyn IParserFactory>,
    logger: &'a Box<dyn ILogger>
  ) -> FileLoader<'a> {
    let mod_state: Box<dyn IModState> = Box::from(ModState::new());
    return FileLoader { 
      config,
      mod_state: Some(mod_state),
      parser_factory,
      logger: logger.create_new()
    }
  }
  
  fn load_country_definitions(&mut self, path_string: String) -> Result<(), ()> {
    let path = Path::new(&path_string).join("common\\country_definitions");
    let directory = match fs::read_dir(path.clone()) {
      Ok(files) => files,
      Err(_) => return self.logger
        .no_entity_found("Country Definitions", path.display()),
    };
    
    for entry in directory {
      if let Ok(entry) = entry {
        let file_path = entry.path();
        let file_text = fs::read_to_string(file_path.clone())
          .unwrap_or(String::new());
        
        let parser = self.parser_factory.create_country_definition_parser();
        let definitions = parser.parse(&file_text);
        
        let definitions = match definitions {
          Err(_) => {
            self.logger.failed_to_parse(path.display()).ok();
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
      Err(_) => return self.logger.no_entity_found("Cultures", path.display()),
      Ok(files) => files,
    };
    
    for entry in directory {
      if let Ok(entry) = entry {
        let file_path = entry.path();
        let file_text = fs::read_to_string(file_path.clone())
        .unwrap_or(String::new());
        
        let parser = self.parser_factory.create_culture_parser();
        let cultures = match parser.parse(&file_text) {
          Err(_) => {
            self.logger.failed_to_parse(path.display()).ok();
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

  fn load_states(&mut self, path_string: String) -> Result<(), ()> {
    let path = Path::new(&path_string).join("common\\history\\states");
    
    let directory = match fs::read_dir(path.clone()) {
      Err(_) => return self.logger.no_entity_found("States", path.display()),
      Ok(files) => files,
    };
    
    for entry in directory {
      if let Ok(entry) = entry {
        let file_path = entry.path();
        let file_text = fs::read_to_string(file_path.clone())
        .unwrap_or(String::new());
        
        let parser = self.parser_factory.create_state_parser();
        let states = match parser.parse(&file_text) {
          Err(_) => {
            self.logger.failed_to_parse(path.display()).ok();
            continue;
          },
          Ok(d) => d,
        };
        
        self.mod_state.as_mut().map(|state| {
          let file_name = Path::file_name(&file_path)
            .map(|file_name| file_name.to_str())
            .flatten()
            .unwrap_or("no_file_name.txt");
          state.set_state_file(
            file_name.to_string(), 
            states
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
      .and(self.load_states(path_string.clone()))
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
      .and(self.load_states(path_string.clone()));
  }
  
  fn load_json(&self) -> Result<(), ()> {
    return Ok(());
  }
  
  fn create_mod_builder(mut self) -> Box<ModBuilder> {
    let mod_state = std::mem::take(&mut self.mod_state);
    let config = self.config.clone_box();
    let mod_builder = ModBuilder::new(
      self.logger.create_new(),
      config, 
      mod_state.unwrap()
    );
    
    Box::from(mod_builder)
  }
}
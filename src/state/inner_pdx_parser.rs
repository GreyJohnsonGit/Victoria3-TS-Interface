use crate::{logger::ILogger, pdx_builder::IPdxBuilder, default_reader::DefaultValueReader, unwrap_or_logger::UnwrapOrLogger, value_reader_ext::IValueReaderExt};
use super::{state_division::StateDivision, state::{IState, State, self}};

const DIVISIONS: &str = "Divisions";
const HOMELANDS: &str = "Homelands"; 
const NAME: &str = "Name";
const ADD_HOMELAND: &str = "add_homeland";
const CREATE_STATE: &str = "create_state";

pub struct InnerStateBuilder {
  name: Option<String>,
  homelands: Option<Vec<String>>,
  divisions: Option<Vec<StateDivision>>,
  logger: Box<dyn ILogger>,
}

impl InnerStateBuilder {
  pub fn new(
    logger: &Box<dyn ILogger>,
  ) -> Self {
    Self { 
      name: None, 
      homelands: None, 
      divisions: None, 
      
      logger: logger.create_new(),
    }
  }
  
  pub fn new_boxed(
    logger: &Box<dyn ILogger>,
  ) -> Box<dyn IPdxBuilder<Box<dyn IState>>> {
    Box::new(InnerStateBuilder::new(logger))
  }
}


impl IPdxBuilder<Box<dyn IState>> 
for InnerStateBuilder
{
  fn apply_root(&mut self, root: &str) {
    self.name = Some(root.to_string());
  }
  
  fn apply(&mut self, token: &str, value: &DefaultValueReader) -> Result<(), ()> {
    match token {
      CREATE_STATE => self.apply_division(value),
      ADD_HOMELAND => self.apply_homeland(value),
      _ => Err(())
    }
  }
  
  fn build(self: Box<Self>) -> Result<Box<dyn IState>, ()> {
    let unwrap_or_logger = UnwrapOrLogger::new(
      &self.logger, 
      state::TYPE_STR
    );

    match unwrap_or_logger.on(NAME, &self.name) {
      Ok(name) => Ok(
        State::new_boxed(
          name, 
          self.homelands.unwrap_or_default(), 
          self.divisions
        )
      ),
      _ => Err(())
    }
  }
  
  fn create_new(&self) -> Box<dyn IPdxBuilder<Box<dyn IState>>> {
    InnerStateBuilder::new_boxed(&self.logger)
  }
}

impl InnerStateBuilder {
  fn apply_homeland(&mut self, value: &DefaultValueReader) -> Result<(), ()> {
    match value.read_string() {
      Ok(value) => match self.homelands {
        Some(ref mut homelands) => Ok(homelands.push(value)),
        None => Ok(self.homelands = Some(vec![value]))
      },
      Err(_) => self.logger.coerce_error(HOMELANDS, "Vec<String>")
    }
  }
  
  fn apply_division(&mut self, value: &DefaultValueReader) -> Result<(), ()> {
    let value = match value.read_object() {
      Ok(value) => value,
      Err(_) => return self.logger.coerce_error(DIVISIONS, "Vec<StateDivision>")
    };
    
    let mut provinces = None::<Vec<String>>;
    let mut state_type = None::<String>;
    let mut country = None::<String>;
    
    for (key, _, value) in value.fields() {
      match key.read_string().as_str() {
        "country" => country = value.read_string().ok(),
        "owned_provinces" => provinces = value.read_string_array().ok(),
        "state_type" => state_type = value.read_string().ok(),
        _ => ()
      }
    }
    
    let division = match (country, provinces) {
      (Some(country), Some(provinces)) => StateDivision { 
        country, provinces, state_type 
      },
      _ => return self.logger.coerce_error(DIVISIONS, "Vec<StateDivision>")
    };
    
    match self.divisions {
      Some(ref mut divisions) => Ok(divisions.push(division)),
      None => Ok(self.divisions = Some(vec![division]))
    }
  }
}

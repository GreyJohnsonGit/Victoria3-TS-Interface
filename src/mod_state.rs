use std::collections::HashMap;
use crate::{country_definition::country_definition::ICountryDefinition, culture::culture::ICulture};

type CountryDefinitionTable = HashMap<String, Vec<Box<dyn ICountryDefinition>>>;
type CultureTable = HashMap<String, Vec<Box<dyn ICulture>>>;

pub trait IModState {
  fn get_country_definitions(&self) -> &CountryDefinitionTable;
  fn get_cultures(&self) -> &CultureTable;
  
  fn set_country_definitions_file(&mut self, 
    file_name: String, 
    definitions: Vec<Box<dyn ICountryDefinition>>
  );
  
  fn set_cultures_file(&mut self, 
    file_name: String, 
    cultures: Vec<Box<dyn ICulture>>
  );
}

pub struct ModState {
  country_definitions: CountryDefinitionTable,
  cultures: CultureTable
}

impl ModState {
  pub fn new() -> ModState {
    ModState {
      country_definitions: HashMap::new(),
      cultures: HashMap::new()
    }
  }
}

impl IModState for ModState {
  fn get_country_definitions(&self) -> &CountryDefinitionTable {
    return &self.country_definitions;
  }
  
  fn get_cultures(&self) -> &CultureTable {
    return &self.cultures;
  }
  
  fn set_country_definitions_file(&mut self, 
    file_name: String, 
    definitions: Vec<Box<dyn ICountryDefinition>>
  ) {
    self.country_definitions.insert(file_name, definitions);
  }

  fn set_cultures_file(&mut self, 
    file_name: String, 
    cultures: Vec<Box<dyn ICulture>>
  ) {
    self.cultures.insert(file_name, cultures);
  }
}
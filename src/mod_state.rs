use std::collections::HashMap;
use crate::{country_definition::country_definition::ICountryDefinition, culture::culture::ICulture, state::state::IState};

type CountryDefinitionTable = HashMap<String, Vec<Box<dyn ICountryDefinition>>>;
type CultureTable = HashMap<String, Vec<Box<dyn ICulture>>>;
type StateTable = HashMap<String, Vec<Box<dyn IState>>>;

pub trait IModState {
  fn get_country_definition_files(&self) -> &CountryDefinitionTable;
  fn set_country_definitions_file(&mut self, 
    file_name: String, 
    definitions: Vec<Box<dyn ICountryDefinition>>
  );
  
  fn get_culture_files(&self) -> &CultureTable;
  fn set_cultures_file(&mut self, 
    file_name: String, 
    cultures: Vec<Box<dyn ICulture>>
  );

  fn get_state_files(&self) -> &StateTable;
  fn set_state_file(&mut self, 
    file_name: String, 
    states: Vec<Box<dyn IState>>
  );
}

pub struct ModState {
  country_definitions: CountryDefinitionTable,
  cultures: CultureTable,
  states: StateTable
}

impl ModState {
  pub fn new() -> ModState {
    ModState {
      country_definitions: HashMap::new(),
      cultures: HashMap::new(),
      states: HashMap::new()
    }
  }
}

impl IModState for ModState {
  fn get_country_definition_files(&self) -> &CountryDefinitionTable {
    return &self.country_definitions;
  }
  
  fn get_culture_files(&self) -> &CultureTable {
    return &self.cultures;
  }

  fn get_state_files(&self) -> &StateTable {
    return &self.states;
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

  fn set_state_file(&mut self, 
    file_name: String, 
    states: Vec<Box<dyn IState>>
  ) {
    self.states.insert(file_name, states);
  }
}
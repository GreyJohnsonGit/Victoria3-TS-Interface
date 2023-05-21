use crate::country_definition::country_definition::ICountryDefinition;

type CountryDefinitionFile = (String, Vec<Box<dyn ICountryDefinition>>);

pub trait IModState {
  fn get_country_definitions(&self) -> &Vec<CountryDefinitionFile>;
  
  fn add_file(&mut self, 
    file_name: String, 
    definitions: Vec<Box<dyn ICountryDefinition>>
  );
}

pub struct ModState {
  country_definitions: Vec<(String, Vec<Box<dyn ICountryDefinition>>)> 
}

impl ModState {
  pub fn new() -> ModState {
    ModState {
      country_definitions: vec![]
    }
  }
}

impl IModState for ModState {
  fn add_file(&mut self, 
    file_name: String, 
    definitions: Vec<Box<dyn ICountryDefinition>>
  ) {
    self.country_definitions.push((file_name, definitions));
  }

  fn get_country_definitions(&self) -> &Vec<CountryDefinitionFile> {
    return &self.country_definitions;
  }
}
use super::country_definition_builder::CountryDefinitionBuilder;

pub trait ICountryDefinitionFactory {
  fn create_builder(&self) -> CountryDefinitionBuilder;
}

pub struct CountryDefinitionFactory {}

impl CountryDefinitionFactory {
  pub fn new() -> CountryDefinitionFactory {
    CountryDefinitionFactory {}
  }

  pub fn new_boxed() -> Box<dyn ICountryDefinitionFactory> {
    let factory = CountryDefinitionFactory::new();
    Box::new(factory)
  }
}

impl ICountryDefinitionFactory for CountryDefinitionFactory {
  fn create_builder(&self) -> CountryDefinitionBuilder {
    return CountryDefinitionBuilder::new();
  }
}
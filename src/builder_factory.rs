use crate::{
  country_definition::country_definition_builder::{CountryDefinitionBuilder, ICountryDefinitionBuilder}, 
  culture::culture_builder::{CultureBuilder, ICultureBuilder}
};

pub trait IBuilderFactory {
  fn country_definition_builder(&self) -> Box<dyn ICountryDefinitionBuilder>;
  fn culture_builder(&self) -> Box<dyn ICultureBuilder>;
}

pub struct BuilderFactory {}

impl BuilderFactory {
  pub fn new() -> BuilderFactory { BuilderFactory {} }
  
  pub fn new_boxed() -> Box<dyn IBuilderFactory> {
    Box::new(BuilderFactory::new())
  }
}

impl IBuilderFactory for BuilderFactory {
  fn country_definition_builder(&self) -> Box<dyn ICountryDefinitionBuilder> {
    Box::new(CountryDefinitionBuilder::new())
  }
  
  fn culture_builder(&self) -> Box<dyn ICultureBuilder> {
    Box::new(CultureBuilder::new())
  }
}
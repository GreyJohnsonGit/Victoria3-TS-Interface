use crate::value_reader_ext::Color;
use super::country_definition::CountryDefinition;

#[derive(Debug)]
pub struct CountryDefinitionBuilder {
  pub tag: Option<String>,
  pub cultures: Option<Vec<String>>,
  pub color: Option<Color>,
  pub country_type: Option<String>,
  pub tier: Option<String>,
  pub religion: Option<String>,
  pub capital: Option<String>
}

impl CountryDefinitionBuilder {
  pub fn new() -> CountryDefinitionBuilder {
    CountryDefinitionBuilder {
      tag: None,
      cultures: None,
      color: None,
      country_type: None,
      tier: None,
      religion: None,
      capital: None
    }
  }

  pub fn build(self) -> CountryDefinition {
    CountryDefinition::new(
      self.tag.unwrap(),
      self.cultures.unwrap(),
      self.color.unwrap(),
      self.country_type.unwrap(),
      self.tier.unwrap(),
      self.religion,
      self.capital
    )
  }
}
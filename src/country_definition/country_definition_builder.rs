use crate::color::Color;
use super::country_definition::CountryDefinition;

pub trait ICountryDefinitionBuilder {
  fn with_tag(&mut self, tag: String) -> &mut Self;
  fn with_cultures(&mut self, cultures: Vec<String>) -> &mut Self;
  fn with_color(&mut self, color: Color) -> &mut Self;
  fn with_country_type(&mut self, country_type: String) -> &mut Self;
  fn with_tier(&mut self, tier: String) -> &mut Self;
  fn with_religion(&mut self, religion: Option<String>) -> &mut Self;
  fn with_capital(&mut self, capital: Option<String>) -> &mut Self;
  fn build(&self) -> CountryDefinition;
}

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
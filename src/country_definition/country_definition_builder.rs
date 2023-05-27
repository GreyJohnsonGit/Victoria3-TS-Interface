use crate::{color::Color, country_tier::CountryTier};
use super::country_definition::{CountryDefinition, ICountryDefinition};
pub trait ICountryDefinitionBuilder {
  fn set_tag(&mut self, tag: String) -> Option<String>;
  fn set_cultures(&mut self, cultures: Vec<String>) -> Option<Vec<String>>;
  fn set_color(&mut self, color: Color) -> Option<Color>;
  fn set_country_type(&mut self, country_type: String) -> Option<String>;
  fn set_tier(&mut self, tier: CountryTier) -> Option<CountryTier>;
  fn set_religion(&mut self, religion: Option<String>) -> Option<String>;
  fn set_capital(&mut self, capital: Option<String>) -> Option<String>;
  fn build(self: Box<Self>) -> Box<dyn ICountryDefinition>;
}

#[derive(Debug)]
pub struct CountryDefinitionBuilder {
  pub tag: Option<String>,
  pub cultures: Option<Vec<String>>,
  pub color: Option<Color>,
  pub country_type: Option<String>,
  pub tier: Option<CountryTier>,
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
}

impl ICountryDefinitionBuilder for CountryDefinitionBuilder {
  fn build(self: Box<Self>) -> Box<dyn ICountryDefinition> {
    Box::new(CountryDefinition::new(
      self.tag.unwrap(),
      self.cultures.unwrap(),
      self.color.unwrap(),
      self.country_type.unwrap(),
      self.tier.unwrap(),
      self.religion,
      self.capital
    ))
  }
  
  fn set_tag(&mut self, tag: String) -> Option<String> {
    std::mem::replace(&mut self.tag, Some(tag))
  }
  
  fn set_cultures(&mut self, cultures: Vec<String>) -> Option<Vec<String>> {
    std::mem::replace(&mut self.cultures, Some(cultures))
  }
  
  fn set_color(&mut self, color: Color) -> Option<Color> {
    std::mem::replace(&mut self.color, Some(color))
  }
  
  fn set_country_type(&mut self, country_type: String) -> Option<String> {
    std::mem::replace(&mut self.country_type, Some(country_type))
  }
  
  fn set_tier(&mut self, tier: CountryTier) -> Option<CountryTier> {
    std::mem::replace(&mut self.tier, Some(tier))
  }
  
  fn set_religion(&mut self, religion: Option<String>) -> Option<String> {
    std::mem::replace(&mut self.religion, religion)
  }
  
  fn set_capital(&mut self, capital: Option<String>) -> Option<String> {
    std::mem::replace(&mut self.capital, capital)
  }
}
use crate::{
  define_get_and_set, 
  declare_get_and_set, 
  country_tier::CountryTier
};
use crate::color::Color;

pub const TYPE_STR: &str = "Country Definition";

pub trait ICountryDefinition {
  declare_get_and_set!(tag, set_tag, String);
  declare_get_and_set!(cultures, set_cultures, Vec<String>);
  declare_get_and_set!(color, set_color, Color);
  declare_get_and_set!(country_type, set_country_type, String);
  declare_get_and_set!(tier, set_tier, CountryTier);
  declare_get_and_set!(religion, set_religion, Option<String>);
  declare_get_and_set!(capital, set_capital, Option<String>);
}

#[derive(PartialEq, Debug)]
pub struct CountryDefinition {
  tag: String,
  cultures: Vec<String>,
  color: Color,
  country_type: String,
  tier: CountryTier,
  religion: Option<String>,
  capital: Option<String>,
}

impl CountryDefinition {
  pub fn new(
    tag: String, 
    cultures: Vec<String>, 
    color: Color, 
    country_type: String, 
    tier: CountryTier, 
    religion: Option<String>, 
    capital: Option<String>
  ) -> CountryDefinition {
    CountryDefinition {
      tag, 
      cultures, 
      color, 
      country_type, 
      tier, 
      religion, 
      capital
    }
  }

  pub fn new_boxed(
    tag: String, 
    cultures: Vec<String>, 
    color: Color, 
    country_type: String, 
    tier: CountryTier, 
    religion: Option<String>, 
    capital: Option<String>
  ) -> Box<dyn ICountryDefinition> {
    Box::new(CountryDefinition::new(
      tag,
      cultures,
      color,
      country_type,
      tier,
      religion,
      capital
    ))
  }
}

impl ICountryDefinition for CountryDefinition {
  define_get_and_set!(tag, set_tag, String);
  define_get_and_set!(cultures, set_cultures, Vec<String>);
  define_get_and_set!(color, set_color, Color);
  define_get_and_set!(country_type, set_country_type, String);
  define_get_and_set!(tier, set_tier, CountryTier);
  define_get_and_set!(religion, set_religion, Option<String>);
  define_get_and_set!(capital, set_capital, Option<String>);
}
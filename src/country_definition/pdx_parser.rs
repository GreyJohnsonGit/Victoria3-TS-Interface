use crate::{color::Color, country_tier::CountryTier, logger::ILogger, value_reader_ext::IValueReaderExt, pdx_builder::IPdxBuilder, default_reader::DefaultValueReader};
use super::country_definition::{ICountryDefinition, CountryDefinition, self};

pub struct CountryDefinitionBuilder {
  tag: Option<String>,
  cultures: Option<Vec<String>>,
  color: Option<Color>,
  country_type: Option<String>,
  tier: Option<CountryTier>,
  religion: Option<String>,
  capital: Option<String>,

  logger: Box<dyn ILogger>
}

impl CountryDefinitionBuilder {
  pub fn new(logger: Box<dyn ILogger>) -> Self {
    Self { 
      tag: None, 
      cultures: None, 
      color: None, 
      country_type: None, 
      tier: None, 
      religion: None, 
      capital: None,

      logger
    }
  }

  pub fn new_boxed(logger: &Box<dyn ILogger>) -> Box<dyn IPdxBuilder<Box<dyn ICountryDefinition>>> {
    Box::new(CountryDefinitionBuilder::new(logger.create_new()))
  }
}

impl IPdxBuilder<Box<dyn ICountryDefinition>> 
for CountryDefinitionBuilder 
{
  fn apply_root(&mut self, root: &str) {
    self.tag = Some(root.to_string());
  }
  
  fn apply(&mut self, token: &str, value: &DefaultValueReader) -> Result<(), ()> {
    match token {
      "country_type" => self.apply_country_type(value),
      "tier" => self.apply_tier(value),
      "color" => self.apply_color(value),
      "cultures" => self.apply_cultures(value),
      "religion" => self.apply_religion(value),
      "capital" => self.apply_capital(value),
      _ => Err(())
    }
  }

  fn build(self: Box<Self>) -> Result<Box<dyn ICountryDefinition>, ()> {
    let mut is_missing_property = false;

    let tag = self.unwrap_or_set_flag(
      &self.tag, &mut is_missing_property, "tag"
    );

    let cultures = self.unwrap_or_set_flag(
      &self.cultures, &mut is_missing_property, "cultures"
    );
    
    let color = self.unwrap_or_set_flag(
      &self.color, &mut is_missing_property, "color"
    );

    let country_type = self.unwrap_or_set_flag(
      &self.country_type, &mut is_missing_property, "country_type"
    );
    
    let tier = self.unwrap_or_set_flag(
      &self.tier, &mut is_missing_property, "tier"
    );
    
    let religion = self.religion.clone();
    let capital = self.capital.clone();

    match is_missing_property {
      true => Err(()),
      false => Ok(CountryDefinition::new_boxed(
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

  fn create_new(&self) -> Box<dyn IPdxBuilder<Box<dyn ICountryDefinition>>> {
    CountryDefinitionBuilder::new_boxed(&self.logger.create_new())
  }
}

impl CountryDefinitionBuilder {
  fn unwrap_or_set_flag<T: Default + Clone>(&self, 
    value: &Option<T>,
    error_flag: &mut bool,
    property: &str, 
  ) -> T {
    match value {
      Some(v) => v.clone(),
      None => {
        let _ = self.logger.missing_property_error(country_definition::TYPE_STR, property);
        *error_flag = true;
        Default::default()
      }
    }
  }

  fn apply_country_type(&mut self, value: &DefaultValueReader) -> Result<(), ()> {
    value.read_string()
      .map(|value| self.country_type = Some(value))
      .map_err(|_| { let _ = self.logger.coerce_error("country_type", "String"); })
  }
  
  fn apply_cultures(&mut self, value: &DefaultValueReader) -> Result<(), ()> {
    value.read_string_array()
      .map(|value| self.cultures = Some(value))
      .map_err(|_| { let _ = self.logger.coerce_error("cultures", "Vec<String>"); })
  }
  
  fn apply_tier(&mut self, value: &DefaultValueReader) -> Result<(), ()> {
    let tier = match value.read_string() {
      Ok(value) => value,
      Err(_) => return self.logger.coerce_error("country_tier", "String")
    };
    
    CountryTier::from(&tier).ok_or(())
      .map(|tier| self.tier = Some(tier))
      .map_err(|_| { let _ = self.logger.coerce_error(&tier, "CountryTier"); })
  }
  
  fn apply_color(&mut self, value: &DefaultValueReader) -> Result<(), ()> {
    value.read_color()
      .map(|value| self.color = Some(value))
      .map_err(|_| { let _ = self.logger.coerce_error("color", "Color"); })
  }
  
  fn apply_religion(&mut self, value: &DefaultValueReader) -> Result<(), ()> {
    value.read_string()
      .map(|value| self.religion = Some(value))
      .map_err(|_| { let _ = self.logger.coerce_error("religion", "String"); })
  }
  
  fn apply_capital(&mut self, value: &DefaultValueReader) -> Result<(), ()> {
    value.read_string()
      .map(|value| self.capital = Some(value))
      .map_err(|_| { let _ = self.logger.coerce_error("capital", "String"); })
  }
}

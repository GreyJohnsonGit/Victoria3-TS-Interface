use std::error::Error;
use jomini::{TextTape, text::ValueReader, Windows1252Encoding};
use crate::{value_reader_ext::IValueReaderExt, builder_factory::IBuilderFactory};
use super::country_definition_builder::ICountryDefinitionBuilder;
use crate::color::Color;

pub trait ICountryDefinition {
  fn tag(&self) -> String;
  fn cultures(&self) -> Vec<String>;
  fn color(&self) -> Color;
  fn country_type(&self) -> String;
  fn tier(&self) -> String;
  fn religion(&self) -> Option<String>;
  fn capital(&self) -> Option<String>;
  
  fn set_tag(&mut self, tag: String);
  fn set_cultures(&mut self, cultures: Vec<String>);
  fn set_color(&mut self, color: Color);
  fn set_country_type(&mut self, country_type: String);
  fn set_tier(&mut self, tier: String);
  fn set_religion(&mut self, religion: Option<String>);
  fn set_capital(&mut self, capital: Option<String>);

  fn as_pdx(&self) -> String;
}

#[derive(PartialEq, Debug)]
pub struct CountryDefinition {
  tag: String,
  cultures: Vec<String>,
  color: Color,
  country_type: String,
  tier: String,
  religion: Option<String>,
  capital: Option<String>,
}

impl CountryDefinition {
  pub fn new(
    tag: String, 
    cultures: Vec<String>, 
    color: Color, 
    country_type: String, 
    tier: String, 
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
  
  pub fn from_pdx(
    text: String,
    factory: &Box<dyn IBuilderFactory>
  ) -> Result<Vec<Box<dyn ICountryDefinition>>, Box<dyn Error>> {
    let tape = match TextTape::from_slice(text.as_bytes()) {
      Err(e) => return Err(Box::from(e)),
      Ok(t) => t,
    };
    
    let reader = tape.windows1252_reader();
    
    let mut definitions: Vec<Box<dyn ICountryDefinition>> = vec![];
    
    for (tag, _, inner) in reader.fields() {
      let mut builder = factory.country_definition_builder();
      builder.set_tag(tag.read_string());
      
      let definition = match inner.read_object() {
        Err(e) => return Err(Box::from(e)),
        Ok(d) => d,
      };
      
      for (key, _, value) in definition.fields() {
        CountryDefinition::token_lookup(
          key.read_string().as_str(), 
          value, 
          &mut builder
        );
      }
      
      definitions.push(builder.build());
    }
    
    return Ok(definitions);
  }
  
  fn token_lookup<'a>(
    token: &str, 
    value: ValueReader<Windows1252Encoding>, 
    builder: &'a mut Box<dyn ICountryDefinitionBuilder>
  ) {
    match token {
      "country_type" => {
        builder.set_country_type(value.read_string().unwrap());
      },
      "tier" => {
        builder.set_tier(value.read_string().unwrap());
      },
      "color" => {
        builder.set_color(value.read_color().unwrap());
      },
      "cultures" => {
        builder.set_cultures(value.read_string_array().unwrap());
      },
      "religion" => {
        builder.set_religion(Some(value.read_string().unwrap()));
      },
      "capital" => {
        builder.set_capital(Some(value.read_string().unwrap()));
      },
      _ => ()
    }
  }
}

impl ICountryDefinition for CountryDefinition {
  fn tag(&self) -> String { self.tag.clone() }
  fn cultures(&self) -> Vec<String> { self.cultures.clone() }
  fn color(&self) -> Color { self.color.clone() }  
  fn country_type(&self) -> String { self.country_type.clone() }
  fn tier(&self) -> String { self.tier.clone() }
  fn religion(&self) -> Option<String> { self.religion.clone() }
  fn capital(&self) -> Option<String> { self.capital.clone() }

  fn set_tag(&mut self, tag: String) { self.tag = tag; }
  fn set_tier(&mut self, tier: String) { self.tier = tier; }
  fn set_color(&mut self, color: Color) { self.color = color; }
  fn set_capital(&mut self, capital: Option<String>) { self.capital = capital; }

  fn set_cultures(&mut self, cultures: Vec<String>) {
    self.cultures = cultures; 
  }

  fn set_country_type(&mut self, country_type: String) { 
    self.country_type = country_type; 
  }

  fn set_religion(&mut self, religion: Option<String>) { 
    self.religion = religion; 
  }

  fn as_pdx(&self) -> String {
    format!(
r#"{} = {{
  color = {}
  country_type = {}
  tier = {}
  cultures = {{ {} }}
  {}
  {}
}}"#, 
      self.tag.clone(), 
      self.color.clone().to_string(), 
      self.country_type.clone(), 
      self.tier.clone(), 
      self.cultures.join(" "),
      self.capital.clone()
        .map(|c| format!("capital = {c}"))
        .unwrap_or("# No Capital".to_string()),
      self.religion.clone()
        .map(|r| format!("religion = {r}"))
        .unwrap_or("# No Religion".to_string())
    )
  }
}

#[cfg(test)]
mod test {
  use crate::{color::Color::*, country_definition::{country_definition::ICountryDefinition}, builder_factory::BuilderFactory};
  use super::CountryDefinition;
  
  #[test]
  fn from_pdx_with_valid_text_should_succeed() {
    // Arrange
    let data = String::from(
      r#"GBR = {
        color = hsv{ 0.99  0.7  0.9 }
        
        country_type = recognized
        
        tier = empire	
        
        cultures = { british scottish }
        capital = STATE_HOME_COUNTIES
      }"#);
      
      let expected: Box<dyn ICountryDefinition> = Box::from(CountryDefinition {
        tag: "GBR".to_string(),
        cultures: vec!["british".to_string(), "scottish".to_string()],
        color: HSV(0.99, 0.7, 0.9),
        country_type: "recognized".to_string(),
        tier: "empire".to_string(),
        religion: None,
        capital: Some("STATE_HOME_COUNTIES".to_string())
      });

      let factory = BuilderFactory::new_boxed();
      
      // Act
      let result = CountryDefinition::from_pdx(data, &factory);
      
      // Assert
      let result = result.unwrap();
      let result = result.get(0).unwrap();
      assert_same_country_definition(&expected, &result);
    }

    fn assert_same_country_definition(
      expected: &Box<dyn ICountryDefinition>, 
      actual: &Box<dyn ICountryDefinition>
    )  {
      assert_eq!(expected.tag(), actual.tag());
      assert_eq!(expected.cultures(), actual.cultures());
      assert_eq!(expected.color(), actual.color());
      assert_eq!(expected.country_type(), actual.country_type());
      assert_eq!(expected.tier(), actual.tier());
      assert_eq!(expected.religion(), actual.religion());
      assert_eq!(expected.capital(), actual.capital());
    }
  }
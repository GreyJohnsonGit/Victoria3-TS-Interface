use jomini::{TextTape, text::ValueReader, Windows1252Encoding, DeserializeError};
use crate::value_reader_ext::{Color, IValueReaderExt};
use super::country_definition_builder::CountryDefinitionBuilder;

#[derive(PartialEq, Debug)]
pub struct CountryDefinition {
  tag: String,
  cultures: Vec<String>,
  color: Color,
  country_type: String,
  tier: String,
  religion: Option<String>,
  capital: Option<String>
}

impl CountryDefinition {
  pub fn new(
    tag: String, cultures: Vec<String>, color: Color, country_type: String, 
    tier: String, religion: Option<String>, capital: Option<String>
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
    text: String
  ) -> Result<Vec<CountryDefinition>, DeserializeError> {
    let tape = TextTape::from_slice(text.as_bytes()).unwrap();
    let reader = tape.windows1252_reader();

    let mut definitions: Vec<CountryDefinition> = vec![];

    for (tag, _, inner) in reader.fields() {
      let mut builder = CountryDefinitionBuilder::new();
      builder.tag = Some(tag.read_string());

      let definition = match inner.read_object() {
        Err(e) => return Err(e),
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

  fn token_lookup<'a>(token: &str, value: ValueReader<Windows1252Encoding>, builder: &'a mut CountryDefinitionBuilder) {
    match token {
      "country_type" => builder.country_type = Some(value.read_string().unwrap()),
      "tier" => builder.tier = Some(value.read_string().unwrap()),
      "color" => builder.color = Some(value.read_color().unwrap()),
      "cultures" => builder.cultures = Some(value.read_string_array().unwrap()),
      "religion" => builder.religion = Some(value.read_string().unwrap()),
      "capital" => builder.capital = Some(value.read_string().unwrap()),
      _ => ()
    }
  }
}

#[cfg(test)]
mod test {
    use crate::value_reader_ext::Color::*;
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

    let expected = CountryDefinition {
      tag: "GBR".to_string(),
      cultures: vec!["british".to_string(), "scottish".to_string()],
      color: HSV(0.99, 0.7, 0.9),
      country_type: "recognized".to_string(),
      tier: "empire".to_string(),
      religion: None,
      capital: Some("STATE_HOME_COUNTIES".to_string())
    };

    // Act
    let result = CountryDefinition::from_pdx(data);

    // Assert
    assert_eq!(*result.unwrap().get(0).unwrap(), expected);
  }
}
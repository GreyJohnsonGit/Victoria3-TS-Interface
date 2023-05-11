use crate::value_reader_ext::Color;

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

  fn build(self) -> CountryDefinition {
    CountryDefinition {
      tag: self.tag.unwrap(),
      cultures: self.cultures.unwrap(),
      color: self.color.unwrap(),
      country_type: self.country_type.unwrap(),
      tier: self.tier.unwrap(),
      religion: self.religion,
      capital: self.capital
    }
  }
}

#[cfg(test)]
mod test {
    use jomini::TextTape;
    use crate::value_reader_ext::IValueReaderExt;
    use crate::country_definition::country_definition::CountryDefinitionBuilder;
    use crate::value_reader_ext::Color::*;
    use super::CountryDefinition;

  #[test]
  fn test() {
    let data = 
br#"GBR = {
	color = hsv{ 0.99  0.7  0.9 }

	country_type = recognized

	tier = empire	
	
	cultures = { british scottish }
	capital = STATE_HOME_COUNTIES
}"#;

    let expected = CountryDefinition {
      tag: "GBR".to_string(),
      cultures: vec!["british".to_string(), "scottish".to_string()],
      color: HSV(0.99, 0.7, 0.9),
      country_type: "recognized".to_string(),
      tier: "empire".to_string(),
      religion: None,
      capital: Some("STATE_HOME_COUNTIES".to_string())
    };

    let tape = TextTape::from_slice(data).unwrap();
    let reader = tape.windows1252_reader();
    let mut storage: Vec<CountryDefinition> = vec![];
    reader.fields().for_each(|(tag, _, inner)| {
      let mut builder = CountryDefinitionBuilder::new();
      builder.tag = Some(tag.read_string());
      let _ = inner.read_object().map(|definition| {
        definition.fields().for_each(|(key, _op, value)| {
          match key.read_string().as_str() {
            "country_type" => builder.country_type = Some(value.read_string().unwrap()),
            "tier" => builder.tier = Some(value.read_string().unwrap()),
            "color" => builder.color = Some(value.read_color().unwrap()),
            "cultures" => builder.cultures = Some(value.read_array().unwrap().values().map(|value| value.read_string().unwrap()).collect()),
            "religion" => builder.religion = Some(value.read_string().unwrap()),
            "capital" => builder.capital = Some(value.read_string().unwrap()),
            _ => ()
          }
        });
      });
      storage.push(builder.build());
    });

    let actual: CountryDefinition = storage.pop().unwrap();
    assert_eq!(actual, expected);
  }
}
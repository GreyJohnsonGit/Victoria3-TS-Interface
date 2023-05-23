use jomini::{TextTape, text::ValueReader, Windows1252Encoding};
use crate::{color::Color, builder_factory::IBuilderFactory, value_reader_ext::IValueReaderExt};
use super::culture_builder::ICultureBuilder;

pub trait ICulture {
  fn string_id(&self) -> String;
  fn color(&self) -> Color;
  fn religion(&self) -> String;
  fn traits(&self) -> Vec<String>;
  fn male_common_first_names(&self) -> Vec<String>;
  fn female_common_first_names(&self) -> Vec<String>;
  fn common_last_names(&self) -> Vec<String>;
  fn noble_last_names(&self) -> Vec<String>;
  fn male_regal_first_names(&self) -> Vec<String>;
  fn female_regal_first_names(&self) -> Vec<String>;
  fn regal_last_names(&self) -> Vec<String>;
  fn ethnicities(&self) -> Vec<String>;
  fn graphics(&self) -> String;
  
  fn set_string_id(&mut self, string_id: String);
  fn set_color(&mut self, color: Color);
  fn set_religion(&mut self, religion: String);
  fn set_traits(&mut self, traits: Vec<String>);
  fn set_male_common_first_names(&mut self, names: Vec<String>);
  fn set_female_common_first_names(&mut self, names: Vec<String>);
  fn set_common_last_names(&mut self, names: Vec<String>);
  fn set_noble_last_names(&mut self, names: Vec<String>);
  fn set_male_regal_first_names(&mut self, names: Vec<String>);
  fn set_female_regal_first_names(&mut self, names: Vec<String>);
  fn set_regal_last_names(&mut self, names: Vec<String>);
  fn set_ethnicities(&mut self, ethnicities: Vec<String>);
  fn set_graphics(&mut self, graphics: String);
  
  fn as_pdx(&self) -> String;
}

#[derive(PartialEq, Debug)]
pub struct Culture {
  string_id: String,
  color: Color,
  religion: String,
  traits: Vec<String>,
  male_common_first_names: Vec<String>,
  female_common_first_names: Vec<String>,
  common_last_names: Vec<String>,
  noble_last_names: Vec<String>,
  male_regal_first_names: Vec<String>,
  female_regal_first_names: Vec<String>,
  regal_last_names: Vec<String>,
  ethnicities: Vec<String>,
  graphics: String,
}

impl Culture {
  pub fn new(
    string_id: String,
    color: Color,
    religion: String,
    traits: Vec<String>,
    male_common_first_names: Vec<String>,
    female_common_first_names: Vec<String>,
    common_last_names: Vec<String>,
    noble_last_names: Vec<String>,
    male_regal_first_names: Vec<String>,
    female_regal_first_names: Vec<String>,
    regal_last_names: Vec<String>,
    ethnicities: Vec<String>,
    graphics: String,
  ) -> Culture {
    return Culture {
      string_id,
      color,
      religion,
      traits,
      male_common_first_names,
      female_common_first_names,
      common_last_names,
      noble_last_names,
      male_regal_first_names,
      female_regal_first_names,
      regal_last_names,
      ethnicities,
      graphics,
    }
  }

  pub fn from_pdx(
    text: String,
    factory: &Box<dyn IBuilderFactory>
  ) -> Result<Vec<Box<dyn ICulture>>, String> {
    let tape = match TextTape::from_slice(text.as_bytes()) {
      Err(e) => return Err(e.to_string()),
      Ok(t) => t,
    };
    
    let reader = tape.windows1252_reader();
    
    let mut cultures: Vec<Box<dyn ICulture>> = vec![];
    
    for (string_id, _, inner) in reader.fields() {
      let mut builder = Box::new(factory.culture_builder());
      builder.set_string_id(string_id.read_string());
      
      let culture = match inner.read_object() {
        Err(e) => return Err(e.to_string()),
        Ok(d) => d,
      };
      
      for (key, _, value) in culture.fields() {
        Culture::token_lookup(
          key.read_string().as_str(), 
          value, 
          &mut builder
        );
      }
      
      cultures.push(Box::from(builder.build()));
    }
    
    return Ok(cultures);
  }

  fn token_lookup<'a>(
    token: &str, 
    value: ValueReader<Windows1252Encoding>, 
    builder: &'a mut Box<dyn ICultureBuilder>
  ) {
    match token {
      "color" => {
        builder.set_color(value.read_color().unwrap());
      },
      "religion" => {
        builder.set_religion(value.read_string().unwrap());
      },
      "traits" => {
        builder.set_traits(value.read_string_array().unwrap());
      },
      "male_common_first_names" => {
        builder.set_male_common_first_names(value.read_string_array().unwrap());
      },
      "female_common_first_names" => {
        builder.set_female_common_first_names(value.read_string_array().unwrap());
      },
      "noble_last_names" => {
        builder.set_noble_last_names(value.read_string_array().unwrap());
      },
      "common_last_names" => {
        builder.set_common_last_names(value.read_string_array().unwrap());
      },
      "ethnicities" => {
        let ethnicities = value
          .read_object().unwrap()
          .fields()
          .map(|(_, _, ethnicity)| ethnicity.read_string())
          .flatten();
        builder.set_ethnicities(ethnicities.collect());
      },
      "graphics" => {
        builder.set_graphics(value.read_string().unwrap());
      },
      _ => ()
    }
  }
}

impl ICulture for Culture {
  fn string_id(&self) -> String { self.string_id.clone() }
  fn color(&self) -> Color { self.color.clone() }
  fn religion(&self) -> String { self.religion.clone() }
  fn traits(&self) -> Vec<String> { self.traits.clone() }
  fn ethnicities(&self) -> Vec<String> { self.ethnicities.clone() }
  fn graphics(&self) -> String { self.graphics.clone() }
  
  fn male_common_first_names(&self) -> Vec<String> {
    self.male_common_first_names.clone()
  }
  
  fn female_common_first_names(&self) -> Vec<String> {
    self.female_common_first_names.clone()
  }
  
  fn common_last_names(&self) -> Vec<String> {
    self.common_last_names.clone()
  }
  
  fn noble_last_names(&self) -> Vec<String> {
    self.noble_last_names.clone()
  }
  
  fn male_regal_first_names(&self) -> Vec<String> {
    self.male_regal_first_names.clone()
  }
  
  fn female_regal_first_names(&self) -> Vec<String> {
    self.female_regal_first_names.clone()
  }
  
  fn regal_last_names(&self) -> Vec<String> {
    self.regal_last_names.clone()
  }
  
  fn set_string_id(&mut self, string_id: String) {
    self.string_id = string_id;
  }
  
  fn set_color(&mut self, color: Color) {
    self.color = color;
  }
  
  fn set_religion(&mut self, religion: String) {
    self.religion = religion;
  }
  
  fn set_traits(&mut self, traits: Vec<String>) {
    self.traits = traits;
  }
  
  fn set_male_common_first_names(&mut self, names: Vec<String>) {
    self.male_common_first_names = names;
  }
  
  fn set_female_common_first_names(&mut self, names: Vec<String>) {
    self.female_common_first_names = names;
  }
  
  fn set_common_last_names(&mut self, names: Vec<String>) {
    self.common_last_names = names;
  }
  
  fn set_noble_last_names(&mut self, names: Vec<String>) {
    self.noble_last_names = names;
  }
  
  fn set_male_regal_first_names(&mut self, names: Vec<String>) {
    self.male_regal_first_names = names;
  }
  
  fn set_female_regal_first_names(&mut self, names: Vec<String>) {
    self.female_regal_first_names = names;
  }
  
  fn set_regal_last_names(&mut self, names: Vec<String>) {
    self.regal_last_names = names;
  }
  
  fn set_ethnicities(&mut self, ethnicities: Vec<String>) {
    self.ethnicities = ethnicities;
  }
  
  fn set_graphics(&mut self, graphics: String) {
    self.graphics = graphics;
  }
  
  fn as_pdx(&self) -> String {
    let to_ethnicity_entry = |e: String| format!("1 = {}", e); 

    format!(
r#"{} = {{
  color = {}
  religion = {}
  traits = {{ {} }}
  male_common_first_names = {{ {} }}
  female_common_first_names = {{ {} }}
  noble_last_names = {{ {} }}
  common_last_names = {{ {} }}
  male_regal_first_names = {{ {} }}
  female_regal_first_names = {{ {} }}
  regal_last_names = {{ {} }}
  ethnicities = {{ 
    {} 
  }}
  graphics = {}
}}"#, 
      self.string_id.clone(),
      self.color.clone().to_string(),
      self.religion.clone(),
      self.traits.join(" "),
      self.male_common_first_names.join(" "),
      self.female_common_first_names.join(" "),
      self.noble_last_names.join(" "),
      self.common_last_names.join(" "),
      self.male_regal_first_names.join(" "),
      self.female_regal_first_names.join(" "),
      self.regal_last_names.join(" "),
      self.ethnicities
        .clone()
        .into_iter()
        .map(|e| to_ethnicity_entry(e))
        .collect::<Vec<String>>()
        .join(""),
      self.graphics.clone()
    )
  }
}
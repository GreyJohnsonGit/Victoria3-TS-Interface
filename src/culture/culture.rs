use jomini::{TextTape, text::ValueReader, Windows1252Encoding};
use crate::{color::Color, value_reader_ext::IValueReaderExt, define_get_and_set, declare_get_and_set};
use super::culture_builder::{ICultureBuilder, CultureBuilder};

pub const TYPE_STR: &str = "Culture";

/// Describes structs that represent the PDX concept of a culture.
pub trait ICulture {
  declare_get_and_set!(string_id, set_string_id, String);
  declare_get_and_set!(traits, set_traits, Vec<String>);
  declare_get_and_set!(ethnicities, set_ethnicities, Vec<String>);
  declare_get_and_set!(graphics, set_graphics, String);
  declare_get_and_set!(color, set_color, Option<Color>);
  declare_get_and_set!(religion, set_religion, Option<String>);
  declare_get_and_set!(male_common_first_names, set_male_common_first_names, Option<Vec<String>>);
  declare_get_and_set!(female_common_first_names, set_female_common_first_names, Option<Vec<String>>);
  declare_get_and_set!(common_last_names, set_common_last_names, Option<Vec<String>>);
  declare_get_and_set!(noble_last_names, set_noble_last_names, Option<Vec<String>>);
  declare_get_and_set!(male_regal_first_names, set_male_regal_first_names, Option<Vec<String>>);
  declare_get_and_set!(female_regal_first_names, set_female_regal_first_names, Option<Vec<String>>);
  declare_get_and_set!(regal_last_names, set_regal_last_names, Option<Vec<String>>);
}

#[derive(PartialEq, Debug)]
pub struct Culture {
  string_id: String,
  traits: Vec<String>,
  ethnicities: Vec<String>,
  graphics: String,
  color: Option<Color>,
  religion: Option<String>,
  male_common_first_names: Option<Vec<String>>,
  female_common_first_names: Option<Vec<String>>,
  common_last_names: Option<Vec<String>>,
  noble_last_names: Option<Vec<String>>,
  male_regal_first_names: Option<Vec<String>>,
  female_regal_first_names: Option<Vec<String>>,
  regal_last_names: Option<Vec<String>>,
}

impl Culture {
  pub fn new(
    string_id: String,
    traits: Vec<String>,
    ethnicities: Vec<String>,
    graphics: String,
    color: Option<Color>,
    religion: Option<String>,
    male_common_first_names: Option<Vec<String>>,
    female_common_first_names: Option<Vec<String>>,
    common_last_names: Option<Vec<String>>,
    noble_last_names: Option<Vec<String>>,
    male_regal_first_names: Option<Vec<String>>,
    female_regal_first_names: Option<Vec<String>>,
    regal_last_names: Option<Vec<String>>,
  ) -> Culture {
    return Culture {
      string_id,
      traits,
      ethnicities,
      graphics,
      color,
      religion,
      male_common_first_names,
      female_common_first_names,
      common_last_names,
      noble_last_names,
      male_regal_first_names,
      female_regal_first_names,
      regal_last_names,
    }
  }

  pub fn new_boxed(
    string_id: String,
    traits: Vec<String>,
    ethnicities: Vec<String>,
    graphics: String,
    color: Option<Color>,
    religion: Option<String>,
    male_common_first_names: Option<Vec<String>>,
    female_common_first_names: Option<Vec<String>>,
    common_last_names: Option<Vec<String>>,
    noble_last_names: Option<Vec<String>>,
    male_regal_first_names: Option<Vec<String>>,
    female_regal_first_names: Option<Vec<String>>,
    regal_last_names: Option<Vec<String>>,
  ) -> Box<dyn ICulture> {
    Box::new(Self::new(
      string_id,
      traits,
      ethnicities,
      graphics,
      color,
      religion,
      male_common_first_names,
      female_common_first_names,
      common_last_names,
      noble_last_names,
      male_regal_first_names,
      female_regal_first_names,
      regal_last_names,
    ))
  }
  
  pub fn from_pdx(
    text: String
  ) -> Result<Vec<Box<dyn ICulture>>, String> {
    let tape = match TextTape::from_slice(text.as_bytes()) {
      Err(e) => return Err(e.to_string()),
      Ok(t) => t,
    };
    
    let reader = tape.windows1252_reader();
    
    let mut cultures: Vec<Box<dyn ICulture>> = vec![];
    
    for (string_id, _, inner) in reader.fields() {
      let mut builder: Box<dyn ICultureBuilder> = Box::new(CultureBuilder::new());
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
  define_get_and_set!(string_id, set_string_id, String);
  define_get_and_set!(traits, set_traits, Vec<String>);
  define_get_and_set!(ethnicities, set_ethnicities, Vec<String>);
  define_get_and_set!(graphics, set_graphics, String);
  define_get_and_set!(color, set_color, Option<Color>);
  define_get_and_set!(religion, set_religion, Option<String>);
  define_get_and_set!(male_common_first_names, set_male_common_first_names, Option<Vec<String>>);
  define_get_and_set!(female_common_first_names, set_female_common_first_names, Option<Vec<String>>);
  define_get_and_set!(common_last_names, set_common_last_names, Option<Vec<String>>);
  define_get_and_set!(noble_last_names, set_noble_last_names, Option<Vec<String>>);
  define_get_and_set!(male_regal_first_names, set_male_regal_first_names, Option<Vec<String>>);
  define_get_and_set!(female_regal_first_names, set_female_regal_first_names, Option<Vec<String>>);
  define_get_and_set!(regal_last_names, set_regal_last_names, Option<Vec<String>>);
}
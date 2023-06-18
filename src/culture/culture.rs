use crate::{
  color::Color, 
  define_get_and_set, 
  declare_get_and_set, 
};

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
    string_id: String, traits: Vec<String>, ethnicities: Vec<String>,
    graphics: String, color: Option<Color>, religion: Option<String>,
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
    string_id: String, traits: Vec<String>, ethnicities: Vec<String>,
    graphics: String, color: Option<Color>, religion: Option<String>,
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
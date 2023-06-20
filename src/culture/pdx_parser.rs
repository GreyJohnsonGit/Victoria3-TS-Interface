use crate::{
  color::Color, 
  logger::ILogger, 
  value_reader_ext::IValueReaderExt, 
  pdx_builder::IPdxBuilder, 
  default_reader::DefaultValueReader, 
  unwrap_or_logger::UnwrapOrLogger, 
  define_applier
};
use super::culture::{ICulture, self, Culture};

const STRING_ID: &str = "string_id";
const TRAITS: &str = "traits";
const ETHNICITIES: &str = "ethnicities";
const GRAPHICS: &str = "graphics";
const COLOR: &str = "color";
const RELIGION: &str = "religion";
const MALE_COMMON_FIRST_NAMES: &str = "male_common_first_names";
const FEMALE_COMMON_FIRST_NAMES: &str = "female_common_first_names";
const COMMON_LAST_NAMES: &str = "common_last_names";
const NOBLE_LAST_NAMES: &str = "noble_last_names";
const MALE_REGAL_FIRST_NAMES: &str = "male_regal_first_names";
const FEMALE_REGAL_FIRST_NAMES: &str = "female_regal_first_names";
const REGAL_LAST_NAMES: &str = "regal_last_names";

pub struct CultureBuilder {
  string_id: Option<String>,
  traits: Option<Vec<String>>,
  ethnicities: Option<Vec<String>>,
  graphics: Option<String>,
  color: Option<Color>,
  religion: Option<String>,
  male_common_first_names: Option<Vec<String>>,
  female_common_first_names: Option<Vec<String>>,
  common_last_names: Option<Vec<String>>,
  noble_last_names: Option<Vec<String>>,
  male_regal_first_names: Option<Vec<String>>,
  female_regal_first_names: Option<Vec<String>>,
  regal_last_names: Option<Vec<String>>,

  logger: Box<dyn ILogger>
}

impl CultureBuilder {
  pub fn new(logger: &Box<dyn ILogger>) -> CultureBuilder {
    CultureBuilder {
      string_id: None,
      color: None,
      religion: None,
      traits: None,
      male_common_first_names: None,
      female_common_first_names: None,
      common_last_names: None,
      noble_last_names: None,
      male_regal_first_names: None,
      female_regal_first_names: None,
      regal_last_names: None,
      ethnicities: None,
      graphics: None,

      logger: logger.create_new()
    }
  }

  pub fn new_boxed(logger: &Box<dyn ILogger>) -> Box<dyn IPdxBuilder<Box<dyn ICulture>>> {
    Box::new(Self::new(logger))
  }
}


impl IPdxBuilder<Box<dyn ICulture>> 
for CultureBuilder 
{
  fn build(self: Box<Self>) -> Result<Box<dyn ICulture>, ()> {
    let unwrap_or_logger = UnwrapOrLogger::new(&self.logger, culture::TYPE_STR);

    match (
      unwrap_or_logger.on(STRING_ID, &self.string_id),
      unwrap_or_logger.on(TRAITS, &self.traits),
      unwrap_or_logger.on(ETHNICITIES, &self.ethnicities),
      unwrap_or_logger.on(GRAPHICS, &self.graphics),
    ) {
      (
        Ok(string_id), 
        Ok(traits), 
        Ok(ethnicities), 
        Ok(graphics)
      ) => Ok(Culture::new_boxed(
        string_id, 
        traits, 
        ethnicities, 
        graphics, 
        self.color.clone(), 
        self.religion.clone(), 
        self.male_common_first_names.clone(), 
        self.female_common_first_names.clone(), 
        self.common_last_names.clone(), 
        self.noble_last_names.clone(), 
        self.male_regal_first_names.clone(), 
        self.female_regal_first_names.clone(), 
        self.regal_last_names.clone(),
      )),
      _ => Err(())
    }
  }
  
  fn apply_root(&mut self, root: &str) {
    self.string_id = Some(root.to_string());
  }
  
  fn apply(&mut self, token: &str, value: &DefaultValueReader) -> Result<(), ()> {
    match token {
      STRING_ID => self.apply_string_id(value),
      COLOR => self.apply_color(value),
      RELIGION => self.apply_religion(value),
      TRAITS => self.apply_traits(value),
      MALE_COMMON_FIRST_NAMES => self.apply_male_common(value),
      FEMALE_COMMON_FIRST_NAMES => self.apply_female_common(value),
      COMMON_LAST_NAMES => self.apply_common_last(value),
      NOBLE_LAST_NAMES => self.apply_noble_last(value),
      MALE_REGAL_FIRST_NAMES => self.apply_male_regal(value),
      FEMALE_REGAL_FIRST_NAMES => self.apply_female_regal(value),
      REGAL_LAST_NAMES => self.apply_regal_last(value),
      ETHNICITIES => self.apply_ethnicities(value),
      GRAPHICS => self.apply_graphics(value),
      _ => Err(())
    }
  }

  fn create_new(&self) -> Box<dyn IPdxBuilder<Box<dyn ICulture>>> {
    Self::new_boxed(&self.logger)
  }
}

impl CultureBuilder {
  define_applier!(apply_string_id, read_string, string_id, STRING_ID, "String");
  define_applier!(apply_color, read_color, color, COLOR, "Color");
  define_applier!(apply_religion, read_string, religion, RELIGION, "String");
  define_applier!(apply_traits, read_string_array, traits, TRAITS, "Vec<String>");
  define_applier!(apply_male_common, read_string_array, male_common_first_names, MALE_COMMON_FIRST_NAMES, "Vec<String>");
  define_applier!(apply_female_common, read_string_array, female_common_first_names, FEMALE_COMMON_FIRST_NAMES, "Vec<String>");
  define_applier!(apply_common_last, read_string_array, common_last_names, COMMON_LAST_NAMES, "Vec<String>");
  define_applier!(apply_noble_last, read_string_array, noble_last_names, NOBLE_LAST_NAMES, "Vec<String>");
  define_applier!(apply_male_regal, read_string_array, male_regal_first_names, MALE_REGAL_FIRST_NAMES, "Vec<String>");
  define_applier!(apply_female_regal, read_string_array, female_regal_first_names, FEMALE_REGAL_FIRST_NAMES, "Vec<String>");
  define_applier!(apply_regal_last, read_string_array, regal_last_names, REGAL_LAST_NAMES, "Vec<String>");
  define_applier!(apply_ethnicities, read_string_array, ethnicities, ETHNICITIES, "Vec<String>");
  define_applier!(apply_graphics, read_string, graphics, GRAPHICS, "String");
}
use crate::color::Color;
use super::culture::{ICulture, Culture};

pub trait ICultureBuilder {
  fn set_string_id(&mut self, string_id: String) -> Option<String>;
  fn set_color(&mut self, color: Color) -> Option<Color>;
  fn set_religion(&mut self, religion: String) -> Option<String>;
  fn set_traits(&mut self, traits: Vec<String>) -> Option<Vec<String>>;
  fn set_male_common_first_names(&mut self, names: Vec<String>) 
  -> Option<Vec<String>>;
  fn set_female_common_first_names(&mut self, names: Vec<String>) 
  -> Option<Vec<String>>;
  fn set_common_last_names(&mut self, names: Vec<String>) 
  -> Option<Vec<String>>;
  fn set_noble_last_names(&mut self, names: Vec<String>) 
  -> Option<Vec<String>>;
  fn set_male_regal_first_names(&mut self, names: Vec<String>) 
  -> Option<Vec<String>>;
  fn set_female_regal_first_names(&mut self, names: Vec<String>) 
  -> Option<Vec<String>>;
  fn set_regal_last_names(&mut self, names: Vec<String>) 
  -> Option<Vec<String>>;
  fn set_ethnicities(&mut self, ethnicities: Vec<String>) 
  -> Option<Vec<String>>;
  fn set_graphics(&mut self, graphics: String) -> Option<String>;
  fn build(self: Box<Self>) -> Box<dyn ICulture>;
}

#[derive(Debug)]
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
}

impl CultureBuilder {
  pub fn new() -> CultureBuilder {
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
    }
  }
  
}

impl ICultureBuilder for CultureBuilder {
  fn set_string_id(&mut self, string_id: String) -> Option<String> {
    std::mem::replace(&mut self.string_id, Some(string_id))
  }
  
  fn set_color(&mut self, color: Color) -> Option<Color> {
    std::mem::replace(&mut self.color, Some(color))
  }
  
  fn set_religion(&mut self, religion: String) -> Option<String> {
    std::mem::replace(&mut self.religion, Some(religion))
  }
  
  fn set_traits(&mut self, traits: Vec<String>) -> Option<Vec<String>> {
    std::mem::replace(&mut self.traits, Some(traits))
  }
  
  fn set_male_common_first_names(&mut self, names: Vec<String>) 
  -> Option<Vec<String>> {
    std::mem::replace(&mut self.male_common_first_names, Some(names))
  }
  
  fn set_female_common_first_names(&mut self, names: Vec<String>) 
  -> Option<Vec<String>> {
    std::mem::replace(&mut self.female_common_first_names, Some(names))
  }
  
  fn set_common_last_names(&mut self, names: Vec<String>) 
  -> Option<Vec<String>> {
    std::mem::replace(&mut self.common_last_names, Some(names))
  }
  
  fn set_noble_last_names(&mut self, names: Vec<String>) 
  -> Option<Vec<String>> {
    std::mem::replace(&mut self.noble_last_names, Some(names))
  }
  
  fn set_male_regal_first_names(&mut self, names: Vec<String>) 
  -> Option<Vec<String>> {
    std::mem::replace(&mut self.male_regal_first_names, Some(names))
  }
  
  fn set_female_regal_first_names(&mut self, names: Vec<String>) 
  -> Option<Vec<String>> {
    std::mem::replace(&mut self.female_regal_first_names, Some(names))
  }
  
  fn set_regal_last_names(&mut self, names: Vec<String>) 
  -> Option<Vec<String>> {
    std::mem::replace(&mut self.regal_last_names, Some(names))
  }
  
  fn set_ethnicities(&mut self, ethnicities: Vec<String>) 
  -> Option<Vec<String>> {
    std::mem::replace(&mut self.ethnicities, Some(ethnicities))
  }
  
  fn set_graphics(&mut self, graphics: String) -> Option<String> {
    std::mem::replace(&mut self.graphics, Some(graphics))
  }
  
  fn build(self: Box<Self>) -> Box<dyn ICulture> {
    Box::from(Culture::new(
      self.string_id.unwrap(),
      self.traits.unwrap(), 
      self.ethnicities.unwrap(), 
      self.graphics.unwrap(),
      self.color, 
      self.religion, 
      self.male_common_first_names, 
      self.female_common_first_names, 
      self.common_last_names, 
      self.noble_last_names, 
      self.male_regal_first_names, 
      self.female_regal_first_names, 
      self.regal_last_names, 
    ))
  }
}
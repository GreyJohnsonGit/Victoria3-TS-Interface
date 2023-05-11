#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait IConfig {
  fn get_vanilla_path(&self) -> String;
  fn get_pdx_path(&self) -> String;
  fn get_json_path(&self) -> String;
  fn get_mod_path(&self) -> String;
  fn get_cache_path(&self) -> String;
  fn clone_box(&self) -> Box<dyn IConfig>;
}

#[derive(Debug, Clone)]
pub struct Config {
  vanilla_path: String,
  pdx_path: String,
  json_path: String,
  mod_path: String,
  cache_path: String
}

impl Config {
  pub fn new(
    vanilla_path: String,
    pdx_path: String,
    json_path: String,
    mod_path: String,
    cache_path: String
  ) -> Config {
    return Config {
      vanilla_path,
      pdx_path,
      json_path,
      mod_path,
      cache_path
    };
  }
}

impl IConfig for Config {
  fn get_vanilla_path(&self) -> String { self.vanilla_path.clone() }
  fn get_pdx_path(&self) -> String { self.pdx_path.clone() }
  fn get_json_path(&self) -> String { self.json_path.clone() }
  fn get_mod_path(&self) -> String { self.mod_path.clone() }
  fn get_cache_path(&self) -> String { self.cache_path.clone() }
  fn clone_box(&self) -> Box<dyn IConfig> { Box::from(self.clone()) }
}
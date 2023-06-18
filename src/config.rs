#[cfg(test)]
use mockall::automock;

/// Describes objects which can store application configuration.
#[cfg_attr(test, automock)]
pub trait IConfig {

  /// Returns the path to the vanilla game directory.
  fn get_vanilla_path(&self) -> String;

  /// Returns the path to the pdx directory.
  fn get_pdx_path(&self) -> String;

  /// Returns the path to the json directory.
  fn get_json_path(&self) -> String;

  /// Returns the path to the mod directory.
  fn get_mod_path(&self) -> String;

  /// Returns the path to the cache directory.
  fn get_cache_path(&self) -> String;

  /// Clones the object into a boxed trait object.
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

  pub fn new_boxed(
    vanilla_path: String,
    pdx_path: String,
    json_path: String,
    mod_path: String,
    cache_path: String
  ) -> Box<dyn IConfig> {
    Box::new(Self::new(vanilla_path, pdx_path, json_path, mod_path, cache_path))
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
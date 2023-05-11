use vicky::file_loader::{FileLoader, IFileLoader};
use vicky::config::{Config, IConfig};

fn main() {
  let config = Config::new(
    String::from("D:/Steam/steamapps/common/Victoria 3/game"),
    String::from("./pdx"),
    String::from("./json"),
    String::from("./mod"),
    String::from("./cache")
  ).clone_box();

  let filer_loader = FileLoader::new(config.clone_box());

  if filer_loader.load_vanilla().is_err() {
    panic!("Failed to load vanilla files!");
  }

  if filer_loader.load_pdx().is_err() {
    panic!("Failed to load pdx files!");
  }

  if filer_loader.load_json().is_err() {
    panic!("Failed to load json files!");
  }

  let mod_builder = filer_loader.create_mod_builder();
  
  if mod_builder.validate().is_err() {
    panic!("Failed to validate mod!");
  }

  if mod_builder.save().is_err() {
    panic!("Failed to save mod!");
  }
}

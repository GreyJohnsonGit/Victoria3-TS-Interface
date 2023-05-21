use vicky::country_definition::country_definition_factory::CountryDefinitionFactory;
use vicky::file_loader::{FileLoader, IFileLoader};
use vicky::config::{Config, IConfig};
use vicky::mod_builder::IModBuilder;

fn main() {
  let config: Box<dyn IConfig> = Box::from(Config::new(
    String::from("D:\\Steam\\steamapps\\common\\Victoria 3\\game"),
    String::from(".\\pdx"),
    String::from(".\\json"),
    String::from(".\\mod"),
    String::from(".\\cache")
  ));
  let country_definition_factory = CountryDefinitionFactory::new_boxed();

  let mut filer_loader = FileLoader::new(
    &config,
    &country_definition_factory
  );

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

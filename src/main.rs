use vicky::builder_factory::BuilderFactory;
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
  
  let builder_factory = BuilderFactory::new_boxed();

  let mut filer_loader = FileLoader::new(
    &config,
    &builder_factory
  );

  let load_result = Ok(())
    .and(filer_loader.load_vanilla())
    .and(filer_loader.load_pdx())
    .and(filer_loader.load_json());
  
  if let Err(e) = load_result {
    panic!("Failed to load files! Error: {}", e);
  }

  let mod_builder = filer_loader.create_mod_builder();
  
  if mod_builder.validate().is_err() {
    panic!("Failed to validate mod!");
  }

  if mod_builder.save().is_err() {
    panic!("Failed to save mod!");
  }
}

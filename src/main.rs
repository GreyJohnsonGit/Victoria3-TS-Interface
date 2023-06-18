use vicky::country_definition::pdx_parser::CountryDefinitionBuilder;
use vicky::culture::pdx_parser::CultureBuilder;
use vicky::file_loader::{FileLoader, IFileLoader};
use vicky::config::Config;
use vicky::logger::Logger;
use vicky::mod_builder::IModBuilder;
use vicky::mod_validator::mod_validator::ModValidator;
use vicky::parser_factory::ParserFactory;

fn main() {
  let logger = Logger::new_boxed();

  let config = Config::new_boxed(
    "D:\\Steam\\steamapps\\common\\Victoria 3\\game".to_string(),
    ".\\pdx".to_string(),
    ".\\json".to_string(),
    ".\\mod".to_string(),
    ".\\cache".to_string()
  );
  
  let parser_factory = ParserFactory::new_boxed(
    CultureBuilder::new_boxed(&logger),
    CountryDefinitionBuilder::new_boxed(&logger)
  );
  let mut filer_loader = FileLoader::new(&config, parser_factory, &logger);

  let load_result = Ok(())
    .and(filer_loader.load_vanilla())
    .and(filer_loader.load_pdx())
    .and(filer_loader.load_json());
  
  if load_result.is_err() {
    logger.fatal_str("Failed to load files.");
    return;
  }

  let mod_builder = filer_loader.create_mod_builder();
  
  let mod_validator = ModValidator::new_boxed(&logger);
  match mod_builder.validate_with(mod_validator) {
    Ok(_) => logger.info_str("Mod is valid."),
    Err(_) => logger.warning_str("Mod failed validation."),
  }

  if mod_builder.save().is_err() {
    logger.fatal_str("Failed to Save.");
  }
}

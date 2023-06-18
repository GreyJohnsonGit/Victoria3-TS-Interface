use crate::{pdx_parser::{IPdxParser, PdxParser}, culture::culture::ICulture, country_definition::country_definition::ICountryDefinition, pdx_builder::IPdxBuilder};

pub trait IParserFactory {
  fn create_culture_parser(&self) -> Box<dyn IPdxParser<Box<dyn ICulture>>>;
  fn create_country_definition_parser(&self) -> Box<dyn IPdxParser<Box<dyn ICountryDefinition>>>;
}

pub struct ParserFactory {
  culture_builder_template: Box<dyn IPdxBuilder<Box<dyn ICulture>>>,
  country_definition_builder_template: Box<dyn IPdxBuilder<Box<dyn ICountryDefinition>>>,
}

impl ParserFactory {
  pub fn new(
    culture_builder_template: Box<dyn IPdxBuilder<Box<dyn ICulture>>>,
    country_definition_builder_template: Box<dyn IPdxBuilder<Box<dyn ICountryDefinition>>>,
  ) -> ParserFactory {
    ParserFactory {
      culture_builder_template: culture_builder_template.create_new(),
      country_definition_builder_template: country_definition_builder_template.create_new(),
    }
  }

  pub fn new_boxed(
    culture_builder_template: Box<dyn IPdxBuilder<Box<dyn ICulture>>>,
    country_definition_builder_template: Box<dyn IPdxBuilder<Box<dyn ICountryDefinition>>>,
  ) -> Box<dyn IParserFactory> {
    Box::new(Self::new(
      culture_builder_template, 
      country_definition_builder_template
    ))
  }
}

impl IParserFactory for ParserFactory {
  fn create_culture_parser(&self) -> Box<dyn IPdxParser<Box<dyn ICulture>>> {
    PdxParser::new_boxed(&self.culture_builder_template)
  }

  fn create_country_definition_parser(&self) -> Box<dyn IPdxParser<Box<dyn ICountryDefinition>>> {
    PdxParser::new_boxed(&self.country_definition_builder_template)
  }
}
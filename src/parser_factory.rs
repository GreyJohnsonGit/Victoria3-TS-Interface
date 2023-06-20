use crate::{pdx_parser::{IPdxParser, PdxParser}, culture::culture::ICulture, country_definition::country_definition::ICountryDefinition, pdx_builder::IPdxBuilder, state::{state::IState, pdx_parser::StateBuilderParser}};

pub trait IParserFactory {
  fn create_culture_parser(&self) -> Box<dyn IPdxParser<Box<dyn ICulture>>>;
  fn create_country_definition_parser(&self) -> Box<dyn IPdxParser<Box<dyn ICountryDefinition>>>;
  fn create_state_parser(&self) -> Box<dyn IPdxParser<Box<dyn IState>>>;
}

pub struct ParserFactory {
  culture_builder_template: Box<dyn IPdxBuilder<Box<dyn ICulture>>>,
  country_definition_builder_template: Box<dyn IPdxBuilder<Box<dyn ICountryDefinition>>>,
  state_builder_template: Box<dyn IPdxBuilder<Vec<Box<dyn IState>>>>,
}

impl ParserFactory {
  pub fn new(
    culture_builder_template: Box<dyn IPdxBuilder<Box<dyn ICulture>>>,
    country_definition_builder_template: Box<dyn IPdxBuilder<Box<dyn ICountryDefinition>>>,
    state_builder_template: Box<dyn IPdxBuilder<Vec<Box<dyn IState>>>>,
  ) -> ParserFactory {
    ParserFactory {
      culture_builder_template,
      country_definition_builder_template,
      state_builder_template
    }
  }

  pub fn new_boxed(
    culture_builder_template: Box<dyn IPdxBuilder<Box<dyn ICulture>>>,
    country_definition_builder_template: Box<dyn IPdxBuilder<Box<dyn ICountryDefinition>>>,
    state_builder_template: Box<dyn IPdxBuilder<Vec<Box<dyn IState>>>>,
  ) -> Box<dyn IParserFactory> {
    Box::new(Self::new(
      culture_builder_template, 
      country_definition_builder_template,
      state_builder_template
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

  fn create_state_parser(&self) -> Box<dyn IPdxParser<Box<dyn IState>>> {
    StateBuilderParser::new_boxed(
      PdxParser::new_boxed(&self.state_builder_template)
    )
  }
}
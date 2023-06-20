use crate::{logger::ILogger, pdx_builder::IPdxBuilder, pdx_parser::{IPdxParser, PdxParser}, default_reader::DefaultValueReader};
use super::{state::IState, inner_pdx_parser::InnerStateBuilder};

/// Wrapper around the default parser to flatten the result from Vec\<Vec\<T\>\> 
/// to Vec\<T\>
pub struct StateBuilderParser {
  parser: Box<dyn IPdxParser<Vec<Box<dyn IState>>>>
}

impl StateBuilderParser {
  pub fn new(
    parser: Box<dyn IPdxParser<Vec<Box<dyn IState>>>>
  ) -> Self {
    Self { parser }
  }
  
  pub fn new_boxed(
    parser: Box<dyn IPdxParser<Vec<Box<dyn IState>>>>
  ) -> Box<dyn IPdxParser<Box<dyn IState>>> {
    Box::new(Self::new(parser))
  }
}

impl IPdxParser<Box<dyn IState>> for StateBuilderParser {
  fn parse(&self, text: &str) -> Result<Vec<Box<dyn IState>>, ()> {
    match self.parser.parse(text) {
      Ok(states) => Ok(states.into_iter().flatten().collect()),
      Err(_) => Err(())
    }
  }
  
  fn parse_reader(&self, reader: &crate::default_reader::DefaultObjectReader) -> Result<Vec<Box<dyn IState>>, ()> {
    match self.parser.parse_reader(reader) {
      Ok(states) => Ok(states.into_iter().flatten().collect()),
      Err(_) => Err(())
    }
  }
  
  fn parse_object(&self, 
    root: &str, 
    reader: &crate::default_reader::DefaultObjectReader
  ) -> Result<Box<dyn IState>, ()> {
    let states = match self.parser.parse_object(root, reader) {
      Ok(states) => states,
      Err(_) => return Err(())
    };

    states.into_iter().nth(0).ok_or(())
  }

  fn create_new(&self) -> Box<dyn IPdxParser<Box<dyn IState>>> {
    Self::new_boxed(self.parser.create_new())
  }
}

pub struct StateBuilder {
  states: Option<Vec<Box<dyn IState>>>,
  state_parser_template: Box<dyn IPdxParser<Box<dyn IState>>>,
  logger: Box<dyn ILogger>
}

impl StateBuilder {
  pub fn new(
    logger: &Box<dyn ILogger>
  ) -> Self {
    let state_parser_template = PdxParser::new_boxed(
      &InnerStateBuilder::new_boxed(logger)
    );
    
    Self { 
      states: None, 
      logger: logger.create_new(),
      state_parser_template
    }
  }
  
  pub fn new_boxed(
    logger: &Box<dyn ILogger>
  ) -> Box<dyn IPdxBuilder<Vec<Box<dyn IState>>>> {
    Box::new(StateBuilder::new(logger))
  }
}

impl IPdxBuilder<Vec<Box<dyn IState>>> 
for StateBuilder
{
  fn apply_root(&mut self, _: &str) {}
  
  fn apply(&mut self, token: &str, value: &DefaultValueReader) -> Result<(), ()> {
    let state_parser = self.state_parser_template.create_new();
    
    let inner = match value.read_object() {
      Ok(inner) => inner,
      Err(_) => return Err(()),
    };
    
    match state_parser.parse_object(token, &inner) {
      Ok(state) => Ok(match self.states {
        Some(ref mut s) => s.push(state),
        None => self.states = Some(vec![state])
      }),
      Err(_) => Err(())
    }
  }
  
  fn build(self: Box<Self>) -> Result<Vec<Box<dyn IState>>, ()> {
    match self.states {
      Some(s) => Ok(s),
      None => Err(())
    }
  }
  
  fn create_new(&self) -> Box<dyn IPdxBuilder<Vec<Box<dyn IState>>>> {
    Self::new_boxed(&self.logger)
  }
}
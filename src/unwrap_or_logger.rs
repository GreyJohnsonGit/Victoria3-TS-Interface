use crate::{logger::ILogger, color::Color};

type UnwrapOrLogFunction<T> = Box<dyn Fn(&str, &Option<T>) -> Result<T, ()>>;

pub struct UnwrapOrLogger {
  type_str: &'static str,
  logger: Box<dyn ILogger>,

  pub str: UnwrapOrLogFunction<String>,
  pub color: UnwrapOrLogFunction<Color>,
  pub vec_str: UnwrapOrLogFunction<Vec<String>>,

}

impl UnwrapOrLogger {
  pub fn new(logger: &Box<dyn ILogger>, type_str: &'static str) -> Self {
    Self {
      type_str,
      logger: logger.clone_boxed(),

      str: build_unwrap_or_log::<String>(&logger, type_str),
      color: build_unwrap_or_log::<Color>(&logger, type_str),
      vec_str: build_unwrap_or_log::<Vec<String>>(&logger, type_str),
    }
  }

  pub fn on<T: Clone>(
    &self,
    property: &str,
    value: &Option<T>
  ) -> Result<T, ()> {
    build_unwrap_or_log::<T>(&self.logger, self.type_str)(property, value)
  }
}

fn build_unwrap_or_log<'a, T: Clone>(
  logger: &Box<dyn ILogger>,
  type_str: &'static str
) -> UnwrapOrLogFunction<T> {
  let logger = logger.clone_boxed();
  Box::new(move |property, value| {
    match value {
      Some(v) => Ok(v.clone()),
      None => {
        let _ = logger.missing_property_error(type_str, property);
        Err(())
      }
    }
  })
}
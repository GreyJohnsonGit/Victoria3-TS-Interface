use crate::config::IConfig;

pub trait ICache {
  fn has(&self, source: String) -> bool;
  fn load(&self, source: String) -> Result<(), String>;
  fn get(&self, source: String) -> Option<String>;
}

pub struct Cache<'a> {
  _config: &'a Box<dyn IConfig>,
}

impl Cache<'_> {
  pub fn provide<'a>(_config: &'a Box<dyn IConfig>) -> Cache<'a> {
    return Cache { _config }
  }
}

impl ICache for Cache<'_> {
  fn has(&self, _source: String) -> bool {
    todo!()
  }
  
  fn load(&self, _source: String) -> Result<(), String> {
    todo!()
  }
  
  fn get(&self, _source: String) -> Option<String> {
    todo!()
  }
}

fn _to_key(source: String, last_change: String) -> String {
  format!("{}-{}", source, last_change)
}

#[cfg(test)]
mod tests {
  mod has {
    #[test]
    fn is_cached_should_true() {

    }
    
    #[test]
    fn is_not_cached_should_false() {
    }

    #[test]
    fn is_not_cached_but_file_exists_should_return_false() {
    }
  }
  
  mod get {
    #[test]
    fn is_cached_should_return_some_data() {
      
    }

    #[test]
    fn is_cached_and_out_of_date_should_some_updated_data() {
      
    }
    
    #[test]
    fn is_not_cached_should_return_none() {
      
    }

    #[test]
    fn is_not_cached_but_file_exists_should_some_data() {
      
    }
  }
  
  mod load {
    #[test]
    fn is_cached_should_ok() {
      
    }
    
    #[test]
    fn is_cached_but_out_of_date_should_ok() {
      
    }

    #[test]
    fn is_not_cached_but_can_be_loaded_should_ok() {
      
    }

    #[test]
    fn is_not_cached_but_cannot_be_loaded_should_err() {
      
    }
  }
}
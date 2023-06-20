use crate::{declare_get_and_set, define_get_and_set};
use super::state_division::StateDivision;

pub const TYPE_STR: &str = "State";

pub trait IState {
  declare_get_and_set!(name, set_name, String);
  declare_get_and_set!(homelands, set_homelands, Vec<String>);
  declare_get_and_set!(divisions, set_divisions, Option<Vec<StateDivision>>);
}

pub struct State {
  name: String,
  homelands: Vec<String>,
  divisions: Option<Vec<StateDivision>>,
}

impl State {
  pub fn new(
    name: String, 
    homelands: Vec<String>, 
    divisions: Option<Vec<StateDivision>>
  ) -> State {
    State { name, homelands, divisions }
  }

  pub fn new_boxed(
    name: String, 
    homelands: Vec<String>, 
    divisions: Option<Vec<StateDivision>>
  ) -> Box<dyn IState> {
    Box::new(State::new(name, homelands, divisions))
  }
}

impl IState for State {
  define_get_and_set!(name, set_name, String);
  define_get_and_set!(homelands, set_homelands, Vec<String>);
  define_get_and_set!(divisions, set_divisions, Option<Vec<StateDivision>>);
}
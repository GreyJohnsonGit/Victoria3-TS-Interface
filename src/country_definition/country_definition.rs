pub fn message() -> &'static str {
  return "Hello, world!"
}

pub struct CountryDefinition {
  tag: String,
  cultures: Vec<String>,
  color: String,
  country_type: String,
  tier: String,
  religion: Option<String>,
  capital: Option<String>
}

impl CountryDefinition {
  pub fn to_json() -> String {
    return String::from("Hello, world!")
  }
}
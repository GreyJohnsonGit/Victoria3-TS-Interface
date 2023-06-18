use crate::to_pdx::IToPdx;
use super::country_definition::ICountryDefinition;

impl IToPdx for Box<dyn ICountryDefinition> {
  fn to_pdx(&self) -> String {
    let tag = self.tag();
    let color = self.color().to_string();
    let country_type = self.country_type();
    let tier = self.tier();
    let tier = tier.to_str();
    let cultures = self.cultures().join(" ");
    let capital = self.capital()
      .map(|c| format!("capital = {}", c))
      .unwrap_or("# No Capital".to_string());
    let religion = self.religion()
      .map(|r| format!("religion = {}", r))
      .unwrap_or("# No Religion".to_string());

    format!(
r#"{} = {{
  color = {}
  country_type = {}
  tier = {}
  cultures = {{ {} }}
  {}
  {}
}}"#, 
      tag,
      color,
      country_type,
      tier,
      cultures,
      capital,
      religion
    )
  }
}
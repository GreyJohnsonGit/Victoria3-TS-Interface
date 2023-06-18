use crate::to_pdx::IToPdx;
use super::culture::ICulture;

impl IToPdx for Box<dyn ICulture> {
  fn to_pdx(&self) -> String {
    let color = self.color().map(
      |c| format!("color = {}", c.to_string())
    ).unwrap_or("# No Color".to_string());
    
    let religion = self.religion().map(
      |r| format!("religion = {}", r)
    ).unwrap_or("# No Religion".to_string());

    let male_common_first_names = self.male_common_first_names().map(
      |n| format!("male_common_first_names = {{ {} }}", n.join(" "))
    ).unwrap_or("# No Male Common First Names".to_string());

    let female_common_first_names = self.female_common_first_names().map(
      |n| format!("female_common_first_names = {{ {} }}", n.join(" "))
    ).unwrap_or("# No Female Common First Names".to_string());

    let noble_last_names = self.noble_last_names().map(
      |n| format!("noble_last_names = {{ {} }}", n.join(" "))
    ).unwrap_or("# No Noble Last Names".to_string());

    let common_last_names = self.common_last_names().map(
      |n| format!("common_last_names = {{ {} }}", n.join(" "))
    ).unwrap_or("# No Common Last Names".to_string());
    
    let male_regal_first_names = self.male_regal_first_names().map(
      |n| format!("male_regal_first_names = {{ {} }}", n.join(" "))
    ).unwrap_or("# No Male Regal First Names".to_string());

    let female_regal_first_names = self.female_regal_first_names().map(
      |n| format!("female_regal_first_names = {{ {} }}", n.join(" "))
    ).unwrap_or("# No Female Regal First Names".to_string());

    let regal_last_names = self.regal_last_names().map(
      |n| format!("regal_last_names = {{ {} }}", n.join(" "))
    ).unwrap_or("# No Regal Last Names".to_string());

    let ethnicities = self.ethnicities()
      .into_iter()
      .map(|e| format!("1 = {}", e))
      .collect::<Vec<String>>().join("");

    format!(
  r#"{} = {{
  {}
  {}
  traits = {{ {} }}
  {}
  {}
  {}
  {}
  {}
  {}
  {}
  ethnicities = {{ 
    {} 
  }}
  graphics = {}
  }}"#, 
      self.string_id(),
      color,
      religion,
      self.traits().join(" "),
      male_common_first_names, 
      female_common_first_names, 
      noble_last_names, 
      common_last_names, 
      male_regal_first_names, 
      female_regal_first_names, 
      regal_last_names,
      ethnicities,
      self.graphics()
    )
  }
}
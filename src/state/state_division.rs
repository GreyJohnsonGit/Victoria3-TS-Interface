use crate::to_pdx::IToPdx;

#[derive(Debug, Clone)]
pub struct StateDivision {
  pub country: String,
  pub provinces: Vec<String>,
  pub state_type: Option<String>,
}

impl IToPdx for StateDivision {
  fn to_pdx(&self) -> String {
    let country = self.country.clone();
    let provinces = self.provinces.clone().join(" ");
    let state_type = match &self.state_type {
        Some(state_type) => format!("state_type = {}", state_type),
        None => "# state_type = incorporated".to_string(),
    };

    format!(
r#"    create_state = {{ 
      country = {}
      {} 
      owned_provinces = {{ {} }} 
    }}"#,
      country,
      state_type,
      provinces
    )
  }
}
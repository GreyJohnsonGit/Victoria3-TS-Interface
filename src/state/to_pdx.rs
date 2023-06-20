use crate::to_pdx::IToPdx;
use super::state::IState;

impl IToPdx for Box<dyn IState> {
  fn to_pdx(&self) -> String {
    let name = self.name();
    let homelands = self.homelands()
      .iter()
      .map(|h| format!("    add_homeland = {}", h))
      .collect::<Vec<String>>()
      .join("\n");
    let divisions = match self.divisions() {
      Some(divisions) => divisions.iter()
        .map(|d| d.to_pdx())
        .collect::<Vec<String>>()
        .join("\n"),
      None => return String::new(),
    };
      
    format!(
r#"  {} = {{
{}
{}
  }}"#, 
      name,
      homelands,
      divisions
    )
  }
}
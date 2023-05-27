pub const CITY_STATE: &str = "city_state";
pub const PRINCIPALITY: &str = "principality";
pub const GRAND_PRINCIPALITY: &str = "grand_principality";
pub const KINGDOM: &str = "kingdom";
pub const EMPIRE: &str = "empire";
pub const HEGEMONY: &str = "hegemony";

#[derive(Debug, PartialEq, Clone)]
pub enum CountryTier {
  CityState,
  Principality,
  GrandPrincipality,
  Kingdom,
  Empire,
  Hegemony,
}

impl CountryTier {
  pub fn from(tier: &str) -> Option<CountryTier> {
    match tier {
      CITY_STATE => Some(CountryTier::CityState),
      PRINCIPALITY => Some(CountryTier::Principality),
      GRAND_PRINCIPALITY => Some(CountryTier::GrandPrincipality),
      KINGDOM => Some(CountryTier::Kingdom),
      EMPIRE => Some(CountryTier::Empire),
      HEGEMONY => Some(CountryTier::Hegemony),
      _ => None
    }
  }

  pub fn to_str(&self) -> &str {
    match self {
      CountryTier::CityState => CITY_STATE,
      CountryTier::Principality => PRINCIPALITY,
      CountryTier::GrandPrincipality => GRAND_PRINCIPALITY,
      CountryTier::Kingdom => KINGDOM,
      CountryTier::Empire => EMPIRE,
      CountryTier::Hegemony => HEGEMONY,
    }
  }
}
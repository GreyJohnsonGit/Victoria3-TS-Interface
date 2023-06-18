pub const CITY_STATE: &str = "city_state";
pub const PRINCIPALITY: &str = "principality";
pub const GRAND_PRINCIPALITY: &str = "grand_principality";
pub const KINGDOM: &str = "kingdom";
pub const EMPIRE: &str = "empire";
pub const HEGEMONY: &str = "hegemony";

/// Country Tiers used to give more powerful countries extra bonuses.
#[derive(Debug, PartialEq, Clone)]
pub enum CountryTier {
  CityState,
  Principality,
  GrandPrincipality,
  Kingdom,
  Empire,
  Hegemony,
}

impl Default for CountryTier {
  fn default() -> Self { CountryTier::CityState }
}

impl CountryTier {
  /// Convert a string to a CountryTier. Returns None if the string is not a 
  /// valid CountryTier.
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

  /// Convert a CountryTier to a string.
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
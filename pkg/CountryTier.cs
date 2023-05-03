using DotNext;

namespace Vicky {
  public enum CountryTier {
    CityState,
    Principality,
    GrandPrincipality,
    Kingdom,
    Empire,
  }

  public static class CountryTierHelper {
    public static string CityState = 
      ToString(CountryTier.CityState).Value;
    public static string Principality =
      ToString(CountryTier.Principality).Value;
    public static string GrandPrincipality =
      ToString(CountryTier.GrandPrincipality).Value;
    public static string Kingdom =
      ToString(CountryTier.Kingdom).Value;
    public static string Empire =
      ToString(CountryTier.Empire).Value;

    public static Optional<CountryTier> FromString(string tier) {
      return tier switch {
        "city_state" => CountryTier.CityState,
        "principality" => CountryTier.Principality,
        "grand_principality" => CountryTier.GrandPrincipality,
        "kingdom" => CountryTier.Kingdom,
        "empire" => CountryTier.Empire,
        _ => new Optional<CountryTier>()
      };
    }

    public static Optional<string> ToString(CountryTier tier) {
      return tier switch {
        CountryTier.CityState => "city_state",
        CountryTier.Principality => "principality",
        CountryTier.GrandPrincipality => "grand_principality",
        CountryTier.Kingdom => "kingdom",
        CountryTier.Empire => "empire",
        _ => new Optional<string>()
      };
    }
  }
}
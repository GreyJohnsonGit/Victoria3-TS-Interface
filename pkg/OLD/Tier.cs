namespace Vicky {
  public enum Tier {
    principality,
    kingdom,
    empire,
    city_state
  }

  public static class TierExtensions {
    public static string ToString(this Tier tier) {
      return tier switch {
        Tier.principality => "principality",
        Tier.kingdom => "kingdom",
        Tier.empire => "empire",
        Tier.city_state => "city_state",
      };
    }
  }
}
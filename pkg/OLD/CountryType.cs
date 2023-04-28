using Optional;

namespace Vicky {
  public enum CountryType {
    Decentralized,
    Recognized,
    Unrecognized,
    Colonial
  }

  public static class CountryTypeExtensions {
    public static string ToString(this CountryType countryType) {
      return countryType switch {
        CountryType.Decentralized => "decentralized",
        CountryType.Recognized => "recognized",
        CountryType.Unrecognized => "unrecognized",
        CountryType.Colonial => "colonial",
      };
    }
  }
}
using DotNext;

namespace Vicky {
  public enum CountryType {
    Unrecognized,
    Decentralized,
    Recognized
  }

  public static class CountryTypeHelper {
    public static string Unrecognized = 
      ToString(CountryType.Unrecognized).Value;
    public static string Decentralized = 
      ToString(CountryType.Decentralized).Value;
    public static string Recognized = 
      ToString(CountryType.Recognized).Value;
    
    public static Optional<CountryType> FromString(string type) {
      return type switch {
        "decentralized" => CountryType.Decentralized,
        "recognized" => CountryType.Recognized,
        "unrecognized" => CountryType.Unrecognized,
        _  => new Optional<CountryType>()
      };
    }

    public static Optional<string> ToString(CountryType type) {
      return type switch {
        CountryType.Decentralized => "decentralized",
        CountryType.Recognized => "recognized",
        CountryType.Unrecognized => "unrecognized",
        _ => new Optional<string>()
      };
    }
  }
}
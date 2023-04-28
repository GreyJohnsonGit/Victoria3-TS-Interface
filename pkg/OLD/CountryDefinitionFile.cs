using Optional;
using System.Text.Json;

namespace Vicky {
  public class CountryDefinitionFile {
    public string Path { get; }
    public CountryDefinition[] CountryDefinitions { get; }

    public CountryDefinitionFile(
      string Path, 
      CountryDefinition[] countryDefinitions
    ) {
      this.Path = Path;
      this.CountryDefinitions = countryDefinitions;
    }

    public static Option<CountryDefinitionFile> FromJSON(string path) {
      var json = File.ReadAllText(path); 
      var jsonCountryDefinitions = JsonSerializer
        .Deserialize<JSONCountryDefinition[]>(json);

      if (jsonCountryDefinitions == null)
        return Option.None<CountryDefinitionFile>();

      var countryDefinitions = jsonCountryDefinitions
        .Select(c => new CountryDefinition(
          c.tag, 
          new Color(c.color[0], c.color[1], c.color[2]), 
          c.type.Into<CountryType>().ValueOr(CountryType.Unrecognized),
          c.tier.Into<Tier>().ValueOr(Tier.principality),
          c.cultures.Select(c => new Culture(c)).ToArray(), 
          new State(c.capital),
          c.religion.IfNotNull(s => new Religion(s))
        ))
        .ToArray();

      return new CountryDefinitionFile(path, countryDefinitions).Some(); 
    }

    override public string ToString() {
      var countryDefinitions = this.CountryDefinitions
        .Aggregate("", (acc, countryDefinition) => $"{acc}\n{countryDefinition}");

      return $@"{countryDefinitions}";
    }

    private class JSONCountryDefinition {
      public string tag { get; }
      public int[] color { get; }
      public string type { get; }
      public string tier { get; }
      public string[] cultures { get; }
      public string? religion { get; }
      public string capital { get; }

      public JSONCountryDefinition(
        string tag, 
        int[] color, 
        string type, 
        string tier, 
        string[] cultures, 
        string? religion, 
        string capital
      ) {
        this.tag = tag;
        this.color = color;
        this.type = type;
        this.tier = tier;
        this.cultures = cultures;
        this.religion = religion;
        this.capital = capital;
      }
    }
  }
}
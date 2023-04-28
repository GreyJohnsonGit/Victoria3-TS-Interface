using Optional;
using System.Text.Json;

namespace Vicky {
  public class CountryCreationFile {
    public string Path { get; }
    public CountryCreation[] CountryCreations { get; }

    public CountryCreationFile(
      string Path, 
      CountryCreation[] countryCreations
    ) {
      this.Path = Path;
      this.CountryCreations = countryCreations;
    }

    public static Option<CountryCreationFile[]> FromJSON(string path) {
      var json = File.ReadAllText(path); 
      var jsonCountryCreations = JsonSerializer
        .Deserialize<JSONCountryCreation[]>(json);

      if (jsonCountryCreations == null)
        return Option.None<CountryCreationFile[]>();

      var countryCreations = jsonCountryCreations
        .Select(c => new CountryCreation(c.tag, c.provinces, c.ai))
        .ToArray();

      return new[] {
        new CountryCreationFile(path, countryCreations)
      }.Some();
    }

    override public string ToString() {
      var countryCreations = this.CountryCreations
        .Aggregate("", (acc, countryCreation) => $"{acc}\n{countryCreation}");

      return $@"{countryCreations}";
    }

    private class JSONCountryCreation {
      public string tag { get; }
      public string[] provinces { get; }
      public bool ai { get; }

      public JSONCountryCreation(string tag, string[] provinces, bool ai) {
        this.tag = tag;
        this.provinces = provinces;
        this.ai = ai;
      }
    }
  }
}
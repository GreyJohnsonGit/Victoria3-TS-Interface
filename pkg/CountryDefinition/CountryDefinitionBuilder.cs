using DotNext;
using Pdoxcl2Sharp;

namespace Vicky {
  public interface ICountryDefinitionBuilder : IParadoxRead {
    public string Tag { set; }
    public int[]? Color { set; }
    public string? Type { set; }
    public string? Tier { set; }
    public string[]? Cultures { set; }
    public string? Religion { set; }
    public string? Capital { set; }
    
    public Optional<CountryDefinition> Build();
  }

  public class CountryDefinitionBuilder : ICountryDefinitionBuilder {
    public string Tag { private get; set; }
    public int[]? Color { private get; set; }
    public string? Type { private get; set; }
    public string? Tier { private get; set; }
    public string[]? Cultures { private get; set; }
    public string? Religion { private get; set; }
    public string? Capital { private get; set; }

    public bool IsConsumed = false;

    public CountryDefinitionBuilder(string tag) {
      this.Tag = tag;
    }

    public Optional<CountryDefinition> Build() {
      if (
        IsConsumed ||
        Cultures == null
      ) return new Optional<CountryDefinition>();

      var type = new Optional<string>(Type)
        .Map(t => CountryTypeHelper.FromString(t))
        .Map(t => (CountryType?)t)
        .Or(null);

      var tier = new Optional<string>(Tier)
        .Map(t => CountryTierHelper.FromString(t))
        .Map(t => (CountryTier?)t)
        .Or(null);
      
      var definition = new CountryDefinition(
        Tag,
        Cultures,
        Color,
        type,
        tier,
        Religion,
        Capital
      );

      IsConsumed = true;
      return definition;
    }

    public void TokenCallback(ParadoxParser parser, string token) {
      switch (token) {
        case "color":
          Color = parser.ReadIntList().ToArray();
          break;
        case "country_type":
          Type = parser.ReadString();
          break;
        case "tier":
          Tier = parser.ReadString();
          break;
        case "cultures":
          Cultures = parser.ReadStringList().ToArray();
          break;
        case "religion":
          Religion = parser.ReadString();
          break;
        case "capital":
          Capital = parser.ReadString();
          break;
      }
    }
  }
}
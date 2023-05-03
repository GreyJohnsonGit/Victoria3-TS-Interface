using DotNext;

namespace Vicky {  
  public interface IJSONCountryDefinition : ISerializable {
    public CountryDefinition AsObject();
  }

  public class JSONCountryDefinition : IJSONCountryDefinition {
    public string tag { get; set; }
    public string[] cultures { get; set; }
    public int[] color { get; set; }
    public string country_type { get; set; }
    public string tier { get; set; }
    public string? religion { get; set; }
    public string? capital { get; set; }

    public JSONCountryDefinition(
      string tag,
      string[] cultures,
      int[] color,
      string country_type,
      string tier,
      string? religion,
      string? capital
    ) {
      this.tag = tag;
      this.cultures = cultures;
      this.color = color;
      this.country_type = country_type;
      this.tier = tier;
      this.religion = religion;
      this.capital = capital;
    }

    public CountryDefinition AsObject() {
      return new CountryDefinition(
        tag,
        cultures,
        color,
        CountryTypeHelper.FromString(country_type).OrNull(),
        CountryTierHelper.FromString(tier).OrNull(),
        religion,
        capital
      );
    }

    public string SerializeToJSON() {
      return DefaultJSONSerializer.Serialize(this);
    }

    public string SerializeToPDX() {
      var religion = new Optional<string>(this.religion);
      var capital = new Optional<string>(this.capital);
      return
@$"{tag} = {{
  color = {{ {color[0]} {color[1]} {color[2]} }}
  country_type = {country_type}
  tier = {tier}
  cultures = {Formatter.Array(cultures)}
  {religion.Map(r => $"religion = {r}").Or("# No Religion")}
  {capital.Map(c => $"capital = {c}").Or("# No Capital")}
}}";
    }
  }
}
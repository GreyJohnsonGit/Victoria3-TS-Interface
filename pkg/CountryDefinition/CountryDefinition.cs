using DotNext;

namespace Vicky {  
  public interface ICountryDefinition : ISerializable {
    public string Tag { get; set; }
    public string[] Cultures { get; set; }
    public int[] Color { get; set; }
    public CountryType Type { get; set; }
    public CountryTier Tier { get; set; }
    public Optional<string> Religion { get; set; }
    public Optional<string> Capital { get; set; }

    public JSONCountryDefinition AsJson();
  }

  public class CountryDefinition : ICountryDefinition {
    public string Tag { get; set; }
    public string[] Cultures { get; set; }
    public int[] Color { get; set; }
    public CountryType Type { get; set; }
    public CountryTier Tier { get; set; }
    public Optional<string> Religion { get; set; }
    public Optional<string> Capital { get; set; }

    public CountryDefinition(
      string tag,
      string[] cultures,
      int[]? color = null,
      CountryType? type = null,
      CountryTier? tier = null,
      string? religion = null,
      string? capital = null
    ) {
      Tag = tag;
      Cultures = cultures;
      Color = color ?? DefaultColor(tag);
      Type = type ?? DefaultType();
      Tier = tier ?? DefaultTier();
      Religion = new Optional<string>(religion);
      Capital = new Optional<string>(capital);
    }

    public CountryDefinition(
      JSONCountryDefinition definition
    ) : this(
      definition.tag,
      definition.cultures,
      definition.color,
      CountryTypeHelper.FromString(definition.country_type).OrNull(),
      CountryTierHelper.FromString(definition.tier).OrNull(),
      definition.religion,
      definition.capital
    ) {}

    public string SerializeToJSON() {
      return DefaultJSONSerializer.Serialize(this.AsJson());
    }

    public string SerializeToPDX() {
      return
@$"{Tag} = {{
  color = {{ {Color[0]} {Color[1]} {Color[2]} }}
  {CountryTypeHelper.ToString(Type).Map(t => $"country_type = {t}").Or("# No Type")}
  {CountryTierHelper.ToString(Tier).Map(t => $"tier = {t}").Or("# No Tier")}
  cultures = {Formatter.Array(Cultures)}
  {Religion.Map(r => $"religion = {r}").Or("# No Religion")}
  {Capital.Map(c => $"capital = {c}").Or("# No Capital")}
}}";
    }

    public JSONCountryDefinition AsJson() {
      return new JSONCountryDefinition(
        Tag,
        Cultures,
        Color,
        CountryTypeHelper.ToString(Type).Or(DefaultTypeRaw()),
        CountryTierHelper.ToString(Tier).Or(DefaultTierRaw()),
        Religion.Or(null),
        Capital.Or(null)
      );
    }

    private CountryType DefaultType() => CountryType.Unrecognized;
    private CountryTier DefaultTier() => CountryTier.CityState;
    private string DefaultTypeRaw() => CountryTypeHelper.Unrecognized;
    private string DefaultTierRaw() => CountryTierHelper.CityState;
    private int[] DefaultColor(string tag) {
      int scale = 256 / 26;
      int offset = 'A';
      return tag
        .Substring(0, 3)
        .Select(c => scale * (c - offset))
        .ToArray();
    }
  }
}
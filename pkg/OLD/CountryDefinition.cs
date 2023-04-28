using Optional;

namespace Vicky {
  public class CountryDefinition {
    public string Tag { get; }
    public Color Color { get; }
    public CountryType Type { get; }
    public Tier Tier { get; }
    public Culture[] Cultures { get; }
    public State Capital { get; }

    public Option<Religion> Religion { get; }

    public CountryDefinition(
      string tag, 
      Color color, 
      CountryType type, 
      Tier tier, 
      Culture[] cultures, 
      State capital,
      Option<Religion>? religion
    ) {
      this.Tag = tag;
      this.Color = color;
      this.Type = type;
      this.Tier = tier;
      this.Cultures = cultures;
      this.Capital = capital;
      this.Religion = religion ?? Option.None<Religion>();
    }
  }
}
namespace Vicky {
  public class CountryCreation {
    public CountryTag Tag { get; }
    public Province[] Provinces { get; }
    public bool AiShouldRelease { get; }

    public CountryCreation(
      string tag,
      string[] provinces, 
      bool aiShouldRelease
    ) {
      this.Tag = new CountryTag(tag);
      this.Provinces = provinces.Select(p => new Province(p)).ToArray();
      this.AiShouldRelease = aiShouldRelease;
    }

    override public string ToString() {
      var provinces = this.Provinces.Aggregate("", (acc, province) => $"{acc} {province.Id}");

      return $@"
{this.Tag} = {{
  provinces = {{ {provinces} }}
  ai_will_do = {{ always = no }}
}}";
    }
  }
}
namespace Vicky {  
  public class JSONCountryDefinition {
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
  }
}
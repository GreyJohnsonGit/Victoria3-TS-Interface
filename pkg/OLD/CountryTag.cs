namespace Vicky {
  public class CountryTag {
    public string Id { get;}

    public CountryTag(string id) {
      this.Id = id;
    }

    override public string ToString() {
      return this.Id;
    }
  }
}
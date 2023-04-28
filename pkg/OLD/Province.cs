namespace Vicky {
  public class Province {
    public string Id { get;}

    public Province(string id) {
      this.Id = id;
    }

    override public string ToString() {
      return this.Id;
    }
  }
}
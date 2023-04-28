namespace Vicky {
  public class Religion {
    public string Sid { get;}

    public Religion(string id) {
      this.Sid = id;
    }

    override public string ToString() {
      return this.Sid;
    }
  }
}
namespace Vicky {
  public class CoatOfArms {
    public string Sid { get; set; }
    public string Body { get; set; }

    public CoatOfArms(string sid, string body) {
      Sid = sid;
      Body = body;
    }

    override public string ToString() {
      return $"{Sid} = {{{Body}\n}}";
    }
  }
}
namespace Vicky {
  public class Context : System.IDisposable {
    public string Path;

    public Context(string path) {
      this.Path = path;
    }

    public Context Add(string path) {
      return new Context($"{this.Path}/{path}");
    }

    public void Dispose() {}

    override public string ToString() {
      return this.Path;
    }
  }
}
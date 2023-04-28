using System.Text.Json;
using Optional;

namespace Vicky {
  public interface ICoatOfArmsFile {
    public string Path { get; set; }
    string ToString();
  }

  public class RawCoatOfArmsFile : ICoatOfArmsFile {
    public string Path { get; set; }
    public string Body { get; set; }

    public RawCoatOfArmsFile(string path, string body) {
      Path = path;
      Body = body;
    }
    
    public static Option<RawCoatOfArmsFile[]> FromJSON(string path) {
      var json = File.ReadAllText(path); 
      var jsonCOA = JsonSerializer.Deserialize<JSONRawCoatOfArmsFile[]>(json);

      if (jsonCOA == null)
        return Option.None<RawCoatOfArmsFile[]>();

      return jsonCOA
        .Select(c => new RawCoatOfArmsFile(c.path, c.body))
        .ToArray()
        .Some();
    }

    override public string ToString() {
      return Body;
    }

    private class JSONRawCoatOfArmsFile {
      public string path { get; }
      public string body { get; }

      public JSONRawCoatOfArmsFile(string path, string body) {
        this.path = path;
        this.body = body;
      }
    }
  }
}
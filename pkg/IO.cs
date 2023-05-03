using DotNext;

namespace Vicky {
  public interface IIO {
    Result<string> Read(string path);
    Result<string> Write(string path, string content);
  }

  public class IO : IIO {
    public Result<string> Read(string path) {
      try {
        return File.ReadAllText(path);
      } catch(Exception e) {
        return new Result<string>(e);
      }
    }

    public Result<string> Write(string path, string content) {
      try {
        File.WriteAllText(path, content);
        return Path.GetFullPath(path);
      } catch (Exception e) {
        return new Result<string>(e);
      }
    }
  }
}
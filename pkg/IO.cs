using DotNext;

namespace Vicky {
  public interface IIO {
    Result<string> Read(string path);
    Result<string> Write(string path, string content);
    Result<Stream> ReadStream(string path);
    Result<string> GetLastWriteTimeUtc(string path);
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

    public Result<Stream> ReadStream(string path) {
      try {
        return new FileStream(path, FileMode.Open);
      } catch (Exception e) {
        return new Result<Stream>(e);
      }
    }

    public Result<string> GetLastWriteTimeUtc(string path) {
      return File.GetLastWriteTimeUtc(path).ToString();
    }
  }
}
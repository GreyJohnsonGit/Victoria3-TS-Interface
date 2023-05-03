using DotNext;
using Vicky;

namespace VickyTests {
  class MockIO : IIO {
    public Dictionary<string, string> Files { get; set; }

    public MockIO() {
      Files = new Dictionary<string, string>();
    }

    public Result<string> Read(string path) {
      return Files[path];
    }

    public Result<string> Write(string path, string content) {
      Files.Add(path, content);
      return path;
    }
  }
}
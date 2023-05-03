using DotNext;

namespace Vicky {
  public static class Formatter {
    public static string IfExists(
      string? value, 
      Func<string, string> formatter,
      string defaultValue = ""
    ) {
      return value == null ? defaultValue : formatter(value);
    }

    public static string IfExists(
      Optional<string> value, 
      Func<string, string> formatter,
      string defaultValue = ""
    ) {
      return value
        .Map<string, string>(v => formatter(v))
        .Or(defaultValue);
    }

    public static string Array(string[] value) {
      return value.Aggregate("{", (a, b) => $"{a} {b}") + " }";
    }
  }
}
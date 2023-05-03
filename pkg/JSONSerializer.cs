using System.Text.Json;

namespace Vicky {
  public class DefaultJSONSerializer {
    public static string Serialize(object? value) {
      return JsonSerializer
        .Serialize(value, new JsonSerializerOptions {
          WriteIndented = true,
          DefaultIgnoreCondition = System.Text.Json.Serialization
            .JsonIgnoreCondition.WhenWritingNull
        })
        .Replace("\r\n", "\n");
    }
  }
}
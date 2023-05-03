using System.Text.Json;
using DotNext;

namespace Vicky {
  public interface IJSONCountryDefinitionFile : ISerializable {
    public CountryDefinitionFile AsObject();
  }

  public class JSONCountryDefinitionFile : IJSONCountryDefinitionFile {
    public string SourcePath { get; set; }
    public string SourceTimestamp { get; set; }
    public WriteStrategy WriteStrategy { get; set; }
    public JSONCountryDefinition[] Data { get; set; }
    
    public JSONCountryDefinitionFile(
      string sourcePath, 
      string sourceTimestamp, 
      WriteStrategy writeStrategy, 
      JSONCountryDefinition[] data
    ) {
      SourcePath = sourcePath;
      SourceTimestamp = sourceTimestamp;
      WriteStrategy = writeStrategy;
      Data = data;
    }

    public CountryDefinitionFile AsObject() {
      return new CountryDefinitionFile(
        SourcePath,
        SourceTimestamp,
        WriteStrategy,
        Data.Select(d => d.AsObject()).ToArray()
      );
    }

    public string SerializeToJSON() {
      return JsonSerializer.Serialize(this);
    }

    public string SerializeToPDX() {
      return this.Data
        .Select(d => d.SerializeToPDX())
        .Aggregate((a, b) => $"{a}\n{b}");
    }
  }
}
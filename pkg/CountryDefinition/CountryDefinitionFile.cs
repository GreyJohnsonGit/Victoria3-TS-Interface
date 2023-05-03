using Pdoxcl2Sharp;
using DotNext;

namespace Vicky {
  public interface ICountryDefinitionFile : 
    IFileDefinition<ICountryDefinition>, IParadoxRead 
  {
    public JSONCountryDefinitionFile AsJson();
  }

  public class CountryDefinitionFile : ICountryDefinitionFile {
    public string SourcePath { get; set; }
    public string SourceTimestamp { get; set; }
    public WriteStrategy WriteStrategy { get; set; }
    public ICountryDefinition[] Data { get; set; }
    
    public CountryDefinitionFile(
      string sourcePath, 
      string sourceTimestamp, 
      WriteStrategy writeStrategy, 
      ICountryDefinition[] data
    ) {
      SourcePath = sourcePath;
      SourceTimestamp = sourceTimestamp;
      WriteStrategy = writeStrategy;
      Data = data;
    }

    public string SerializeToJSON() {
      return this.AsJson().SerializeToJSON();
    }

    public string SerializeToPDX() {
      return this.AsJson().SerializeToJSON();
    }

    public void TokenCallback(ParadoxParser parser, string token) {
      var builder = parser.Parse(new CountryDefinitionBuilder(token));
      var definition = builder.Build();
      if (definition.HasValue)
        Data = Data.Append(definition.Value).ToArray();
    }

    public JSONCountryDefinitionFile AsJson() {
      return new JSONCountryDefinitionFile(
        SourcePath,
        SourceTimestamp,
        WriteStrategy,
        Data.Select(d => d.AsJson()).ToArray()
      );
    }
  }
}
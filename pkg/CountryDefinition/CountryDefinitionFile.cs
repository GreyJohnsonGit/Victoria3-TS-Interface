using Pdoxcl2Sharp;
using DotNext;

namespace Vicky {
  public interface ICountryDefinitionFile : 
    IFileDefinition<ICountryDefinition>, 
    IParadoxRead {
    public JSONCountryDefinitionFile AsJson();
  }

  public class CountryDefinitionFile : ICountryDefinitionFile {
    public string SourcePath { get; set; }
    public string SourceTimestamp { get; set; }
    public WriteStrategy WriteStrategy { get; set; }
    public ICountryDefinition[] Data { get; set; }
    
    private ICountryDefinitionFactory _factory;

    public CountryDefinitionFile(
      string sourcePath, 
      string sourceTimestamp, 
      WriteStrategy writeStrategy, 
      ICountryDefinition[] data,
      ICountryDefinitionFactory? factory = null
    ) {
      SourcePath = sourcePath;
      SourceTimestamp = sourceTimestamp;
      WriteStrategy = writeStrategy;
      Data = data;
      _factory = factory ?? new CountryDefinitionFactory(new IO());
    }

    public CountryDefinitionFile(
      JSONCountryDefinitionFile file,
      ICountryDefinitionFactory? factory = null
    ) : this(
      file.SourcePath,
      file.SourceTimestamp,
      file.WriteStrategy,
      file.Data.Select(d => new CountryDefinition(d)).ToArray(),
      factory
    ) {}

    public string SerializeToJSON() {
      return DefaultJSONSerializer.Serialize(this.AsJson());
    }

    public string SerializeToPDX() {
      return this.Data
        .Select(d => d.SerializeToPDX())
        .Aggregate((a, b) => $"{a}\n{b}");
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
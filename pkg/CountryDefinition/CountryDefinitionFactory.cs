using Pdoxcl2Sharp;
using System.Text.Json;
using DotNext;

namespace Vicky {
  public interface ICountryDefinitionFactory : 
    IDeserializationFactory<CountryDefinitionFile> {
    public CountryDefinitionBuilder CreateBuilder(string tag);
  }

  public class CountryDefinitionFactory : ICountryDefinitionFactory {
    private IIO _io;

    public CountryDefinitionFactory(IIO io) {
      _io = io;
    }

    public CountryDefinitionBuilder CreateBuilder(string tag) {
      return new CountryDefinitionBuilder(tag);
    }

    public Optional<CountryDefinitionFile> DeserializeJSON(string path) {
      var file = _io.Read(path).Or(null);
      if (file == null)
        return null;

      var json = JsonSerializer.Deserialize<JSONCountryDefinitionFile>(file);
      if (json == null)
        return null;

      return new CountryDefinitionFile(json);
    }

    public Optional<CountryDefinitionFile> DeserializePDX(string path) {
      var file = _io.ReadStream(path).Or(null);
      if (file == null)
        return Optional.None<CountryDefinitionFile>();

      using (file) {
        var lastWriteTime = _io.GetLastWriteTimeUtc(path).Or(null);
        if (lastWriteTime == null)
          return Optional.None<CountryDefinitionFile>();

        return ParadoxParser.Parse(
          file, 
          new CountryDefinitionFile(
            path, 
            lastWriteTime,
            WriteStrategy.Initialize, 
            new CountryDefinition[] {}
          )
        );
      }
    }
  }
}
using Pdoxcl2Sharp;
using System.Text.Json;
using DotNext;

namespace Vicky {
  public interface ICountryDefinitionFactory : 
    IDeserializationFactory<CountryDefinitionFile> 
  {}

  public class CountryDefinitionFactory : ICountryDefinitionFactory {
    private IIO _io;

    public CountryDefinitionFactory(IIO io) {
      _io = io;
    }

    public Optional<CountryDefinitionFile> DeserializeJSON(string path) {
      var file = _io.Read(path).Or(null);
      if (file == null)
        return null;

      var json = JsonSerializer.Deserialize<JSONCountryDefinitionFile>(file);
      if (json == null)
        return null;

      return json.AsObject();
    }

    public Optional<CountryDefinitionFile> DeserializePDX(string path) {
      using (FileStream file = new FileStream(path, FileMode.Open)) {
        return ParadoxParser.Parse(
          file, 
          new CountryDefinitionFile(
            path, 
            File.GetLastWriteTimeUtc(path).ToString(), 
            WriteStrategy.Initialize, 
            new CountryDefinition[] {}
          )
        );
      }
    }
  }
}
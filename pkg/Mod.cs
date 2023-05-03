using DotNext;

namespace Vicky {
  public interface IMod {
    public Result<string> Write();
  }

  public class Mod : IMod {
    private IMetadata _metadata { get; set; }
    private Optional<CountryDefinitionFile[]> _countryDefinitionFiles { get; set; }

    public Mod(
      IMetadata metadata,
      Optional<CountryDefinitionFile[]> countryDefinitionFiles
    ) {
      _metadata = metadata;
      _countryDefinitionFiles = countryDefinitionFiles;
    }

    public Result<string> Write() {
      Console.WriteLine($"Writing...");

      if (Directory.Exists("./mod"))
        Directory.Delete("./mod", true);

      Directory.CreateDirectory("./mod");

      // Metadata
      Directory.CreateDirectory("./mod/.metadata");
      File.WriteAllText(
        "./mod/.metadata/metadata.json", 
        _metadata.SerializeToJSON()
      );

      File.Copy(
        $"./assets/{_metadata.Thumbnail}", 
        $"./mod/.metadata/{_metadata.Thumbnail}"
      );

      // Country Definitions
      _countryDefinitionFiles.Then(files => {
        Directory.CreateDirectory("./mod/common/country_definitions");
        foreach (var file in files) {
          var path = Path.GetFileName(file.SourcePath);
          File.WriteAllText(
            $"./mod/common/country_definitions/{file.SourcePath}",
            file.SerializeToPDX()
          );
        }
      });

      return Path.GetFullPath("../mod");
    }
  }
}
using Vicky;

namespace AOD {
  class Program {
    public static void Main(string[] args) {
      var modBuilder = new ModBuilder(
        AOD.Metadata.Get()
      );

      modBuilder.CountryDefinitionFiles = new CountryDefinitionFile[] {
        new CountryDefinitionFile(
          "fake_file.txt", 
          "None", 
          WriteStrategy.Initialize,
          new CountryDefinition[] {
            new CountryDefinition(
              tag: "AOD",
              color: new[] { 12, 23, 23 },
              type: CountryType.Unrecognized,
              tier: CountryTier.CityState,
              cultures: new[] { "AOD" },
              religion: null,
              capital: null
            )
          }
        )
      };

      var result = modBuilder.Build();

      if (modBuilder.Errors.TryGet(out var errors)) {
        foreach (var error in errors) {
          Console.WriteLine(error);
        }
      }

      if (result.TryGet(out var mod)) {
        mod.Write();
      }
    }

    private static Func<T> onFailedToLoad<T>(string path) {
      return () => throw new Exception($"Failed to load {path}");
    }
  }
}
namespace Vicky {
  public class ModBuilder {
    public void BuildMod(
      Metadata metaData,
      EventFile[] eventFiles,
      ICoatOfArmsFile[] coatOfArmsFiles,
      CountryCreationFile[] countryCreations
    ) {
      var context = new Context(".");

      using (var modCtx = context.Add("mod")) {
        this.Mod(modCtx);
        this.MetaData(modCtx.Add(".metadata"), metaData);
        this.Events(modCtx.Add("events"), eventFiles);
        using (var commonCtx = modCtx.Add("common")) {
          this.Common(commonCtx);
          this.CoatOfArms(commonCtx.Add("coat_of_arms"), coatOfArmsFiles);
          this.CountryCreations(commonCtx.Add("country_creation"), countryCreations);
        }
      }
    }

    private void CreateDirectoryIfNotExists(string path) {
      if (!Directory.Exists(path))
        Directory.CreateDirectory(path);
    }

    private void Mod(Context context) {
      if (Directory.Exists($"{context}"))
        Directory.Delete($"{context}", true);
      
      Directory.CreateDirectory($"{context}");
    }

    private void MetaData(Context context, Metadata metaData) {
      CreateDirectoryIfNotExists($"{context}");
      
      File.WriteAllText(
        $"{context}/metadata.json", 
        metaData.ToString()
      );
      
      File.Copy(
        $"Assets/{metaData.thumbnail}", 
        $"{context}/thumbnail.png", 
        true
      );
    }

    private void Events(Context context, EventFile[] eventFiles) {
      CreateDirectoryIfNotExists($"{context}");
      foreach (var eventFile in eventFiles) {
        var directory = Path.GetDirectoryName(eventFile.Path) ?? String.Empty;
        
        using (var eventCtx = context.Add(directory)) {
          CreateDirectoryIfNotExists($"{eventCtx}");

          File.WriteAllText(
            $"{eventCtx}/{Path.GetFileName(eventFile.Path)}", 
            eventFile.ToString()
          );
        }
      }
    }

    private void Common(Context context) {
      CreateDirectoryIfNotExists($"{context}");
    }

    private void CoatOfArms(Context context, ICoatOfArmsFile[] coatOfArmsFiles) {
      CreateDirectoryIfNotExists($"{context}");
      foreach (var coatOfArmsFile in coatOfArmsFiles) {
        var directory = Path.GetDirectoryName(coatOfArmsFile.Path) 
          ?? String.Empty;
        
        using (var coatOfArmsCtx = context.Add(directory)) {
          CreateDirectoryIfNotExists($"{coatOfArmsCtx}");

          File.WriteAllText(
            $"{coatOfArmsCtx}/{Path.GetFileName(coatOfArmsFile.Path)}", 
            coatOfArmsFile.ToString()
          );
        }
      }
    }

    private void CountryCreations(Context context, CountryCreationFile[] countryCreations) {
      CreateDirectoryIfNotExists($"{context}");
      foreach (var countryCreation in countryCreations) {
        var directory = Path.GetDirectoryName(countryCreation.Path) 
          ?? String.Empty;
        
        using (var countryCreationCtx = context.Add(directory)) {
          CreateDirectoryIfNotExists($"{countryCreationCtx}");

          File.WriteAllText(
            $"{countryCreationCtx}/{Path.GetFileName(countryCreation.Path)}", 
            countryCreation.ToString()
          );
        }
      }
    }
  }
}
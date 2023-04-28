using Vicky;

class Program {
  public static void Main(string[] args) {
    var metadata = new Metadata(
      name: "Age of Discovery 1444",
      id: "2923418734",
      version: "0.1",
      supported_game_version: "",
      thumbnail: "thumbnail.png",
      short_description: "Age of discovery 1444",
      tags: new[] { 
        "Alternative History", 
        "Conversion", 
        "Map", 
        "Expansion" 
      },
      relationships: new string[] {},
      game_custom_data: new Dictionary<string, bool> { 
        { "multiplayer_synchronized", true } 
      }
    );

    var coatOfArmsPath = @"Source/AOD/CoatOfArms.json";
    var countryCreationPath = @"Source/AOD/CountryCreation.json";
    var eventsPath = @"Source/AOD/Events.json";

    var coatOfArmsFiles = RawCoatOfArmsFile
      .FromJSON(coatOfArmsPath)
      .ValueOr(onFailedToLoad<RawCoatOfArmsFile[]>(coatOfArmsPath));

    var countryCreationFiles = CountryCreationFile
      .FromJSON(countryCreationPath)
      .ValueOr(onFailedToLoad<CountryCreationFile[]>(countryCreationPath));

    var eventFiles = EventFile
      .FromJSON(eventsPath)
      .ValueOr(onFailedToLoad<EventFile[]>(eventsPath));

    var modBuilder = new ModBuilder();
    modBuilder.BuildMod(
      metadata,
      eventFiles,
      coatOfArmsFiles,
      countryCreationFiles
    );
  }

  private static Func<T> onFailedToLoad<T>(string path) {
    return () => throw new Exception($"Failed to load {path}");
  }
}


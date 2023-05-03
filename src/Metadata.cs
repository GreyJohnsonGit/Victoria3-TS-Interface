namespace AOD {
  public static class Metadata {
    public static Vicky.IMetadata Get() {
      return new Vicky.Metadata(
        name: "Age of Discovery 1444",
        id: "2923418734",
        version: "0.1",
        supportedGameVersion: "",
        thumbnail: "thumbnail.png",
        shortDescription: "Age of discovery 1444",
        tags: new[] { 
          "Alternative History", 
          "Conversion", 
          "Map", 
          "Expansion" 
        },
        relationships: new string[] {},
        gameCustomData: new Dictionary<string, bool> { 
          { "multiplayer_synchronized", true } 
        }
      );
    }
  }
}
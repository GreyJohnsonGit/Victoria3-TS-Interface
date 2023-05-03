using System.Text.Json;

namespace Vicky {
  public interface IMetadata : ISerializable {
    public string Name { get; set; }
    public string Id { get; set; }
    public string Version { get; set; }
    public string SupportedGameVersion { get; set; }
    public string Thumbnail { get; set; }
    public string ShortDescription { get; set; }
    public string[] Tags { get; set; }
    public string[] Relationships { get; set; }
    public Dictionary<string, bool> GameCustomData { get; set; }
  }

  class JSONMetadata {
    public string name { get; set; }
    public string id { get; set; }
    public string version { get; set; }
    public string supported_game_version { get; set; }
    public string thumbnail { get; set; }
    public string short_description { get; set; }
    public string[] tags { get; set; }
    public string[] relationships { get; set; }
    public Dictionary<string, bool> game_custom_data { get; set; }

    public JSONMetadata(
      string name,
      string id,
      string version,
      string supportedGameVersion,
      string thumbnail,
      string shortDescription,
      string[] tags,
      string[] relationships,
      Dictionary<string, bool> gameCustomData
    ) {
      this.name = name;
      this.id = id;
      this.version = version;
      this.supported_game_version = supportedGameVersion;
      this.thumbnail = thumbnail;
      this.short_description = shortDescription;
      this.tags = tags;
      this.relationships = relationships;
      this.game_custom_data = gameCustomData;
    }
  }

  public class Metadata : IMetadata {
    public string Name {
      get => _json.name;
      set => _json.name = value;
    }
    public string Id {
      get => _json.id;
      set => _json.id = value;
    }
    public string Version {
      get => _json.version;
      set => _json.version = value;
    }
    public string SupportedGameVersion {
      get => _json.supported_game_version;
      set => _json.supported_game_version = value;
    }
    public string Thumbnail {
      get => _json.thumbnail;
      set => _json.thumbnail = value;
    }
    public string ShortDescription {
      get => _json.short_description;
      set => _json.short_description = value;
    }
    public string[] Tags {
      get => _json.tags;
      set => _json.tags = value;
    }
    public string[] Relationships {
      get => _json.relationships;
      set => _json.relationships = value;
    }
    public Dictionary<string, bool> GameCustomData {
      get => _json.game_custom_data;
      set => _json.game_custom_data = value;
    }

    private JSONMetadata _json;

    public Metadata(
      string name,
      string id,
      string version,
      string supportedGameVersion,
      string thumbnail,
      string shortDescription,
      string[] tags,
      string[] relationships,
      Dictionary<string, bool> gameCustomData
    ) {
      this._json = new JSONMetadata(
        name,
        id,
        version,
        supportedGameVersion,
        thumbnail,
        shortDescription,
        tags,
        relationships,
        gameCustomData
      );
    }

    public string SerializeToJSON() {
      return JsonSerializer.Serialize(_json);
    }

    public string SerializeToPDX() {
      return "";
    }
  }
}
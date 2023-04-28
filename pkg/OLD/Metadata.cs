using System.Text.Json;

namespace Vicky {
  public class Metadata {
    public string name { get; }
    public string id { get; }
    public string version { get; }
    public string supported_game_version { get; }
    public string thumbnail { get; }
    public string short_description { get; }
    public string[] tags { get; }
    public string[] relationships { get; }
    public Dictionary<string, bool> game_custom_data { get; }

    public Metadata(
      string name, 
      string id, 
      string version, 
      string supported_game_version, 
      string thumbnail, 
      string short_description, 
      string[] tags, 
      string[] relationships, 
      Dictionary<string, bool> game_custom_data
    ) {
      this.name = name;
      this.id = id;
      this.version = version;
      this.supported_game_version = supported_game_version;
      this.thumbnail = thumbnail;
      this.short_description = short_description;
      this.tags = tags;
      this.relationships = relationships;
      this.game_custom_data = game_custom_data;
    }

    override public string ToString() {
      return JsonSerializer.Serialize(
        this, 
        new JsonSerializerOptions { 
          WriteIndented = true 
        }
      );
    }
  }
}
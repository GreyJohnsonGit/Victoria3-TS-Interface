using Optional;
using System.Text.Json;

namespace Vicky {
  public class EventFile {
    public string Path;
    public Event[] Events;

    public EventFile(string path, Event[] events) {
      this.Path = path;
      this.Events = events;
    }

    public static Option<EventFile[]> FromJSON(string path) {
      var json = File.ReadAllText(path); 
      var jsonEvents = JsonSerializer.Deserialize<JSONEventFile[]>(json);

      if (jsonEvents == null)
        return Option.None<EventFile[]>();

      return jsonEvents
        .Select(
          f => new Vicky.EventFile(
            f.path, 
            f.events
              .Select(e => new Vicky.Event(e))
              .ToArray()
        ))
        .ToArray()
        .Some();
    }

    override public string ToString() {
      return String.Join('\n', this.Events.Select(e => e.Body));
    }

    private class JSONEventFile {
      public string path { get; }
      public string[] events { get; }

      public JSONEventFile(string path, string[] events) {
        this.path = path;
        this.events = events;
      }
    }
  }
}
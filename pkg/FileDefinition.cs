namespace Vicky {
  public interface IFileDefinition<TData> : ISerializable where TData : ISerializable {
    string SourcePath { get; set; }
    string SourceTimestamp { get; set; }
    WriteStrategy WriteStrategy { get; set; }
    TData[] Data { get; set; }
  }

  public enum WriteStrategy {
    Initialize,
    Overwrite,
    Merge
  }
}
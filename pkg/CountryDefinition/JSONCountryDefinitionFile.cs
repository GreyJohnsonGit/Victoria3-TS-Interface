namespace Vicky {
  public class JSONCountryDefinitionFile {
    public string SourcePath { get; set; }
    public string SourceTimestamp { get; set; }
    public WriteStrategy WriteStrategy { get; set; }
    public JSONCountryDefinition[] Data { get; set; }
    
    public JSONCountryDefinitionFile(
      string sourcePath, 
      string sourceTimestamp, 
      WriteStrategy writeStrategy, 
      JSONCountryDefinition[] data
    ) {
      SourcePath = sourcePath;
      SourceTimestamp = sourceTimestamp;
      WriteStrategy = writeStrategy;
      Data = data;
    }
  }
}
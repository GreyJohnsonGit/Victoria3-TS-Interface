using Vicky;
using Xunit;

namespace VickyTests {
  public class CountryDefinitionFactoryTests {
    [Fact]
    public void Constructor_Primary_ShouldSucceed() {
      var io = new IO();
      var factory = new CountryDefinitionFactory(io);
    }

    [Fact]
    public void DeserializeJSON_ValidPathAndContent_ShouldSucceed() {
      // Arrange
      var io = new MockIO();
      var path = "path";
      var timestamp = "timestamp";
      var writeStrategy = WriteStrategy.Initialize;
      var text = @$"{{
        ""SourcePath"": ""{path}"",
        ""SourceTimestamp"": ""{timestamp}"",
        ""WriteStrategy"": {(int) writeStrategy},
        ""Data"": [
          {{
            ""tag"": ""tag"",
            ""cultures"": [""culture""],
            ""color"": [0, 0, 0],
            ""country_type"": ""{CountryTypeHelper.Decentralized}"",
            ""tier"": ""{CountryTierHelper.CityState}"",
            ""religion"": ""religion"",
            ""capital"": ""capital""
          }}
        ]
      }}";
      io.Files.Add("path", text);
      var factory = new CountryDefinitionFactory(io);

      // Act
      var result = factory.DeserializeJSON(path);
      
      // Assert
      Assert.True(result.HasValue);
      var definitionFile = result.Value;
      Assert.Equal(path, definitionFile.SourcePath);
      Assert.Equal(timestamp, definitionFile.SourceTimestamp);
      Assert.Equal(writeStrategy, definitionFile.WriteStrategy);
      Assert.NotEmpty(definitionFile.Data);
      var definition = definitionFile.Data[0];
      Assert.Equal("tag", definition.Tag);
      Assert.Equal("culture", definition.Cultures[0]);
      Assert.Equal(new[] {0, 0, 0}, definition.Color);
      Assert.Equal(CountryType.Decentralized, definition.Type);
      Assert.Equal(CountryTier.CityState, definition.Tier);
      Assert.Equal("religion", definition.Religion);
      Assert.Equal("capital", definition.Capital);
    }
  }
}
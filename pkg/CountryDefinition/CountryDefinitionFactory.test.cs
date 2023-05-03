using Vicky;
using Xunit;
using Moq;
using DotNext;
using System.Text;

namespace VickyTests {
  public class CountryDefinitionFactoryTests {
    [Fact]
    public void Constructor_Primary_ShouldSucceed() {
      var mockIo = new Mock<IIO>();
      var factory = new CountryDefinitionFactory(mockIo.Object);
    }

    [Fact]
    public void DeserializeJSON_ValidPathAndContent_ShouldSucceed() {
      // Arrange
      var writeStrategy = WriteStrategy.Initialize;
      var path = "path";
      var timestamp = "timestamp";
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

      var mockIo = new Mock<IIO>();
      mockIo
        .Setup(i => i.Read(It.IsAny<string>()))
        .Returns(text);
      var factory = new CountryDefinitionFactory(mockIo.Object);

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

    [Fact]
    public void DeserializeJSON_InvalidPath_ShouldFail() {
      // Arrange
      var path = "path";

      var mockIo = new Mock<IIO>();
      mockIo
        .Setup(i => i.Read(It.IsAny<string>()))
        .Returns(new Result<string>(new Exception()));
      var factory = new CountryDefinitionFactory(mockIo.Object);

      // Act
      var result = factory.DeserializeJSON(path);
      
      // Assert
      Assert.False(result.HasValue);
    }

    [Fact]
    public void DeserializePDX_ValidPathAndData_ShouldSucceed() {
      // Arrange
      var writeStrategy = WriteStrategy.Initialize;
      var path = "path";
      var timestamp = "timestamp";
      var text = @$"AFF = {{
        cultures = {{ culture }}
        color = {{ 0 0 0 }}
        country_type = {CountryTypeHelper.Decentralized}
        tier = {CountryTierHelper.CityState}
        religion = religion
        capital = capital
      }}";

      var mockIo = new Mock<IIO>();
      mockIo
        .Setup(i => i.ReadStream(It.IsAny<string>()))
        .Returns(new MemoryStream(Encoding.UTF8.GetBytes(text)));
      mockIo
        .Setup(i => i.GetLastWriteTimeUtc(It.IsAny<string>()))
        .Returns(timestamp);
      var factory = new CountryDefinitionFactory(mockIo.Object);

      // Act
      var result = factory.DeserializePDX(path);

      // Assert
      Assert.True(result.HasValue);
      var definitionFile = result.Value;
      Assert.Equal(path, definitionFile.SourcePath);
      Assert.Equal(timestamp, definitionFile.SourceTimestamp);
      Assert.Equal(writeStrategy, definitionFile.WriteStrategy);
      Assert.NotEmpty(definitionFile.Data);
      var definition = definitionFile.Data[0];
      Assert.Equal("AFF", definition.Tag);
      Assert.Equal("culture", definition.Cultures[0]);
      Assert.Equal(new[] {0, 0, 0}, definition.Color);
      Assert.Equal(CountryType.Decentralized, definition.Type);
      Assert.Equal(CountryTier.CityState, definition.Tier);
      Assert.Equal("religion", definition.Religion);
      Assert.Equal("capital", definition.Capital);
    }
  }
}
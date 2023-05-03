using Xunit;
using Vicky;
using Moq;

namespace VickyTests {
  public class CountryDefinitionFileTests {
    [Fact]
    public void Constructor_EmptyData_ShouldSucceed() {
      var file = new CountryDefinitionFile(
        "path",
        "timestamp",
        WriteStrategy.Initialize,
        new CountryDefinition[] {}
      );

      Assert.Equal("path", file.SourcePath);
      Assert.Equal("timestamp", file.SourceTimestamp);
      Assert.Equal(WriteStrategy.Initialize, file.WriteStrategy);
      Assert.Empty(file.Data);
    }

    [Fact]
    public void SerializeToJSON_WithData_ShouldSucceed() {
      // Arrange
      var path = "path";
      var timestamp = "timestamp";
      var strategy = WriteStrategy.Initialize;

      var mockDefinition = new Mock<ICountryDefinition>();
      mockDefinition.Setup(d => d.SerializeToJSON()).Returns("{}");
      var file = new CountryDefinitionFile(
        path,
        timestamp,
        strategy,
        new ICountryDefinition[] {}
      );

      // Act
      var json = file.SerializeToJSON();

      // Assert
      Assert.Equal(
@$"{{
  ""SourcePath"": ""path"",
  ""SourceTimestamp"": ""timestamp"",
  ""WriteStrategy"": 0,
  ""Data"": []
}}", json);
    }

    [Fact]
    public void SerializeToPDX_WithData_ShouldSucceed() {
      // Arrange
      var path = "path";
      var timestamp = "timestamp";
      var strategy = WriteStrategy.Initialize;

      var mockDefinition = new Mock<ICountryDefinition>();
      mockDefinition.Setup(d => d.SerializeToPDX()).Returns("definition");
      var file = new CountryDefinitionFile(
        path,
        timestamp,
        strategy,
        new ICountryDefinition[] { 
          mockDefinition.Object,
          mockDefinition.Object,
          mockDefinition.Object
        }
      );

      // Act
      var pdx = file.SerializeToPDX();

      // Assert
      Assert.Equal("definition\ndefinition\ndefinition", pdx);
    }
  }
}
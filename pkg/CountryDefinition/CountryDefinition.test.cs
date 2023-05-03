using Xunit;
using Vicky;

namespace VickyTests {
  public class CountryDefinitionTests {
    public static string tag = "ABC";
    public static string[] cultures = new[]{ "Birit", "Bokku" };
    
    [Fact]
    public void Constructor_WithMinimalArguments_ShouldSucceed() {
      var countryDefinition = new CountryDefinition(tag, cultures);
    }

    [Fact]
    public void Constructor_WithMinimalArguments_ShouldUseDefaults() {
      // Arrange + Act
      var countryDefinition = new CountryDefinition(tag, cultures);

      // Assert
      Assert.Equal(tag, countryDefinition.Tag);
      Assert.Equal(cultures, countryDefinition.Cultures);
      Assert.Equal(new[]{ 0, 9, 18 }, countryDefinition.Color);
      Assert.Equal(CountryType.Unrecognized, countryDefinition.Type);
      Assert.Equal(CountryTier.CityState, countryDefinition.Tier);
      Assert.False(countryDefinition.Religion.HasValue);
      Assert.False(countryDefinition.Capital.HasValue);
    }

    [Fact]
    public void SerializeToJSON_WithPartialDefinition_ShouldSucceed() {
      // Arrange
      var countryDefinition = new CountryDefinition(tag, cultures);

      // Act
      var json = countryDefinition.SerializeToJSON();

      // Assert
      Assert.Equal( 
@$"{{
  ""tag"": ""{tag}"",
  ""cultures"": [
    ""{cultures[0]}"",
    ""{cultures[1]}""
  ],
  ""color"": [
    0,
    9,
    18
  ],
  ""country_type"": ""unrecognized"",
  ""tier"": ""city_state""
}}", json
      );
    }

    [Fact]
    public void SerializeToPDX_WithPartialDefinition_ShouldSucceed() {
      // Arrange
      var countryDefinition = new CountryDefinition(tag, cultures);

      // Act
      var pdx = countryDefinition.SerializeToPDX();

      // Assert
      Assert.Equal(
$@"{tag} = {{
  color = {{ 0 9 18 }}
  country_type = unrecognized
  tier = city_state
  cultures = {{ {cultures[0]} {cultures[1]} }}
  # No Religion
  # No Capital
}}", pdx
      );
    }
  }
}
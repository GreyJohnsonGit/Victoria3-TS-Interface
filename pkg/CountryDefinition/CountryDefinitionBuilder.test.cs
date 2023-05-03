using Xunit;
using Vicky;
using Pdoxcl2Sharp;
using System.Text;

namespace VickyTests {
  public class CountryDefinitionBuilderTests {
    public static string tag = "ABC";
    public static string[] cultures = new[]{ "Birit", "Bokku" };

    [Fact]
    public void Constructor_WithTag_ShouldSucceed() {
      var builder = new CountryDefinitionBuilder(tag);
    }

    [Fact]
    public void Build_WithNullCultures_ShouldReturnEmpty() {
      // Arrange
      var builder = new CountryDefinitionBuilder("Bool");
      builder.Cultures = null;

      // Act
      var definition = builder.Build();
      
      // Assert
      Assert.True(definition.IsNone());
    }

    [Fact]
    public void Build_WithMinimalArguments_ShouldSucceed() {
      // Arrange
      var builder = new CountryDefinitionBuilder(tag);
      builder.Cultures = cultures;

      // Act
      var definition = builder.Build();
      
      // Assert
      Assert.True(definition.IsSome());
    }

    [Fact]
    public void Build_WithFullDefinition_ShouldHaveAllData() {
      // Arrange
      var builder = new CountryDefinitionBuilder(tag);
      builder.Cultures = cultures;
      builder.Color = new[]{ 1, 2, 3 };
      builder.Type = CountryTypeHelper.Decentralized;
      builder.Tier = CountryTierHelper.CityState;
      builder.Religion = "catholic";
      builder.Capital = "STATE_BALINTO";

      // Act
      var definition = builder.Build();
      
      // Assert
      Assert.True(definition.IsSome());
      Assert.Equal(tag, definition.Value.Tag);
      Assert.Equal(cultures, definition.Value.Cultures);
      Assert.Equal(new[]{ 1, 2, 3 }, definition.Value.Color);
      Assert.Equal(CountryType.Decentralized, definition.Value.Type);
      Assert.Equal(CountryTier.CityState, definition.Value.Tier);
      Assert.Equal("catholic", definition.Value.Religion);
      Assert.Equal("STATE_BALINTO", definition.Value.Capital);
    }

    [Fact]
    public void TokenCallback_WithPDXInput_ShouldBeDeserialized() {
      // Arrange
      var text = 
$@"
	color = {{ 47 91 18 }}

	country_type = recognized

	tier = empire
	
	cultures = {{ russian }}
	capital = STATE_INGRIA
";

      // Act
      var builder = ParadoxParser.Parse<CountryDefinitionBuilder>(
        new MemoryStream(Encoding.UTF8.GetBytes(text)), 
        new CountryDefinitionBuilder(tag)
      );
      var result = builder.Build();

      // Assert
      Assert.True(result.IsSome());
      var definition = result.Value;
      Assert.Equal(tag, definition.Tag);
      Assert.Equal(new[]{ 47, 91, 18 }, definition.Color);
      Assert.Equal(CountryType.Recognized, definition.Type);
      Assert.Equal(CountryTier.Empire, definition.Tier);
      Assert.Equal(new[]{ "russian" }, definition.Cultures);
      Assert.Equal("STATE_INGRIA", definition.Capital);
    }

    [Fact]
    public void Build_SecondBuildAttempt_ShouldBeNone() {
      // Arrange
      var builder = new CountryDefinitionBuilder(tag);
      builder.Cultures = cultures;

      // Act
      var definition = builder.Build();
      var secondDefinition = builder.Build();
      
      // Assert
      Assert.True(definition.IsSome());
      Assert.True(secondDefinition.IsNone());
    }
  }
}
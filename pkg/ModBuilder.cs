using DotNext;

namespace Vicky {
  public interface IModBuilder {
    public IMetadata Metadata { get; set; }
    public Optional<CountryDefinitionFile[]> CountryDefinitionFiles { get; set; }
    public Optional<string[]> Errors { get; }

    public Result<IMod> Build();
  }

  public class ModBuilder : IModBuilder {
    public IMetadata Metadata { get; set; }
    public Optional<CountryDefinitionFile[]> CountryDefinitionFiles { get; set; }
    public Optional<string[]> Errors { get; private set; }

    public ModBuilder(IMetadata metadata) {
      Metadata = metadata;
    }

    public Result<IMod> Build() {
      if (Errors.HasValue)
        return new Result<IMod>();

      return new Mod(
        Metadata,
        CountryDefinitionFiles
      );
    }
    
    private void AddError(string errorMessage) {
        Errors
          .Or(new string[] {})
          .Append(errorMessage);
    }
  }
}
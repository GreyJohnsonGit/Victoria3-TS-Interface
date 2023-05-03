using DotNext;

namespace Vicky { 
  public interface IDeserializationFactory<TData> where TData : ISerializable {
    public Optional<TData> DeserializePDX(string path);
    public Optional<TData> DeserializeJSON(string path);
  }
}
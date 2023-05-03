namespace Vicky {
  public interface ISerializable {
    public string SerializeToJSON();
    public string SerializeToPDX();
  }
}
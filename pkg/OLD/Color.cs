namespace Vicky {
  public class Color {
    public int Red { get; }
    public int Green { get; }
    public int Blue { get; }

    public Color(int red, int green, int blue) {
      this.Red = red;
      this.Green = green;
      this.Blue = blue;
    }

    public override string ToString() {
      return $"{this.Red} {this.Green} {this.Blue}";
    }
  }
}
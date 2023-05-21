pub const HSV: &'static str = "hsv";
pub const HSV360: &'static str = "hsv360";
pub const RGB: &'static str = "rgb";

#[derive(PartialEq, Debug, Clone)]
pub enum Color {
  HSV(f32, f32, f32),
  HSV360(u16, u16, u16),
  RGB(u8, u8, u8),
}

impl Color {
  pub fn to_string(self) -> String {
    match self {
      Color::HSV(h, s, v) => format!("hsv {{ {} {} {} }}", h, s, v),
      Color::HSV360(h, s, v) => format!("hsv360 {{ {} {} {} }}", h, s, v),
      Color::RGB(r, g, b) => format!("rgb {{ {} {} {} }}", r, g, b),
    }
  }
}
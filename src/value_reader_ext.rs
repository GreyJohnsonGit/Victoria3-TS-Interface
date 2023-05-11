use jomini::{Windows1252Encoding, text::{ValueReader, ValuesIter}, DeserializeError};

const HSV: &'static str = "hsv";
const RGB: &'static str = "rgb";

#[derive(PartialEq, Debug)]
pub enum Color {
  HSV(f32, f32, f32),
  RGB(u8, u8, u8),
}

pub trait IValueReaderExt {
  fn read_color(&self) -> Result<Color, DeserializeError>;
}

impl IValueReaderExt for ValueReader<'_, '_, Windows1252Encoding> {
  /// Reads a color from a value reader.
  /// 
  /// @note This function will often Ok with a default color instead of Error in
  /// a few cases.
  /// 
  /// @internal Ensure that the function properly propagates errors
  fn read_color(&self) -> Result<Color, DeserializeError> {
    let mut format: String = String::new();

    let mut values = match self.read_array().map(|v| v.values()) {
      Ok(v) => v,
      Err(e) => return Err(e)
    };
    
    while let Some(value) = values.next() {
      if let Ok(s) = value.read_string() {
        format = s;
        continue;
      }

      if let Ok(v) = value.read_array() {
        match format.as_str() {
          HSV => return read_hsv(&mut v.values()),
          RGB => return read_rgb(&mut v.values()),
          _ => {}
        };
      }
    }

    return Ok(Color::RGB(0, 0, 0));
  }
}

fn read_rgb(
  values: &mut ValuesIter<'_, '_, Windows1252Encoding>
) -> Result<Color, DeserializeError> {
  let mut rgb: Vec<u8> = vec![];
  while let Some(v) = values.next() {
    if let Ok(n) = v.read_scalar().map(|v| v.to_u64()) {
      let value: u8 = match n {
        Err(e) => return Err(e.into()),
        Ok(v) => v.try_into().unwrap_or(0)
      };
      rgb.push(value);
    }
  };

  return Ok(Color::RGB(
    *rgb.get(0).unwrap_or(&0),
    *rgb.get(1).unwrap_or(&0),
    *rgb.get(2).unwrap_or(&0)
  ));
}

fn read_hsv(
  values: &mut ValuesIter<'_, '_, Windows1252Encoding>
) -> Result<Color, DeserializeError> {
  let mut hsv: Vec<f32> = vec![];
  while let Some(v) = values.next() {
    if let Ok(n) = v.read_scalar().map(|v| v.to_f64()) {
      let value = match n {
        Err(e) => return Err(e.into()),
        Ok(v) => v as f32
      };
      hsv.push(value);
    }
  };

  return Ok(Color::HSV(
    *hsv.get(0).unwrap_or(&0.0),
    *hsv.get(1).unwrap_or(&0.0),
    *hsv.get(2).unwrap_or(&0.0)
  ));
}

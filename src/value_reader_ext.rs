use jomini::{Windows1252Encoding, text::ValueReader, DeserializeError, Scalar};
use serde::de::Error;

const HSV: &'static str = "hsv";
const HSV360: &'static str = "hsv360";
const RGB: &'static str = "rgb";

#[derive(PartialEq, Debug)]
pub enum Color {
  HSV(f32, f32, f32),
  HSV360(u16, u16, u16),
  RGB(u8, u8, u8),
}

pub trait IValueReaderExt {
  /// Reads a color from a value reader.
  /// 
  /// @note This function will read the following formats:
  ///  - { Scalar, Scalar, Scalar }
  /// - { "hsv", { Scalar, Scalar, Scalar } }
  /// - { "hsv360", { Scalar, Scalar, Scalar } }
  /// - { "rgb", { Scalar, Scalar, Scalar } }
  fn read_color(&self) -> Result<Color, DeserializeError>;

  /// Reads a string array from a value reader.
  fn read_string_array(&self) -> Result<Vec<String>, DeserializeError>;
}

impl IValueReaderExt for ValueReader<'_, '_, Windows1252Encoding> {
  fn read_color(&self) -> Result<Color, DeserializeError> {
    let mut values: Vec<_> = match self.read_array() {
      Err(e) => return Err(e),
      Ok(v) => v.values().collect(),
    };

    // { Scalar, Scalar, Scalar }
    if values.len() == 3 {
      return read_rgb(&mut values);
    }

    // Format { Scalar, Scalar, Scalar }
    let format = values[0].read_string();
    let array = values[1].read_array();

    match (format, array) {
      (Ok(f), Ok(a)) => {
        match f.as_str() {
          HSV => return read_hsv(&mut a.values().collect()),
          RGB => return read_rgb(&mut a.values().collect()),
          HSV360 => return read_hsv360(&mut a.values().collect()),
          _ => return Err(DeserializeError::custom("Invalid color format")),
        };
      },
      (Err(e), _) => return Err(e),
      (_, Err(e)) => return Err(e),
    };
  }

  fn read_string_array(&self) -> Result<Vec<String>, DeserializeError> {
    let array = match self.read_array() {
      Ok(v) => v,
      Err(e) => return Err(e)
    };

    let mut output_array: Vec<String> = vec![];
    for value in array.values() {
      match value.read_string() {
        Ok(s) => output_array.push(s),
        Err(e) => return Err(e),
      };
    }

    return Ok(output_array);
  }
}

fn read_color_type<'a, T>(
  values: &'a mut Vec<ValueReader<Windows1252Encoding>>,
  to_number: fn(Scalar<'a>) -> Result<T, DeserializeError>,
  to_color: fn(Vec<T>) -> Color
) -> Result<Color, DeserializeError> {
  let mut color_data: Vec<T> = vec![];
  
  for value in values {
    let scalar = match value.read_scalar() {
      Err(e) => return Err(e),
      Ok(n) => n,
    };

    let number: T = match to_number(scalar) {
      Err(e) => return Err(e),
      Ok(n) => n,
    };

    color_data.push(number);
  };

  return Ok(to_color(color_data));
}

fn read_rgb<'a>(
  values: &'a mut Vec<ValueReader<'a, '_, Windows1252Encoding>>
) -> Result<Color, DeserializeError> {
  return read_color_type::<u8>(
    values, 
    |i| match i.to_u64() {
      Err(e) => Err(e.into()),
      Ok(n) => Ok(u8::try_from(n).unwrap_or(0)),
    },
    |n| Color::RGB(
      *n.get(0).unwrap_or(&0),
      *n.get(1).unwrap_or(&0),
      *n.get(2).unwrap_or(&0)
    )
  )
}

fn read_hsv<'a>(
  values: &'a mut Vec<ValueReader<'a, '_, Windows1252Encoding>>
) -> Result<Color, DeserializeError> {
  return read_color_type::<f32>(
    values, 
    |i| match i.to_f64() {
      Err(e) => Err(e.into()),
      Ok(n) => Ok(n as f32),
    },
    |n| Color::HSV(
      *n.get(0).unwrap_or(&0f32),
      *n.get(1).unwrap_or(&0f32),
      *n.get(2).unwrap_or(&0f32)
    )
  )
}

fn read_hsv360<'a>(
  values: &'a mut Vec<ValueReader<'a, '_, Windows1252Encoding>>
) -> Result<Color, DeserializeError> {
  return read_color_type::<u16>(
    values, 
    |i| match i.to_u64() {
      Err(e) => Err(e.into()),
      Ok(n) => Ok(u16::try_from(n).unwrap_or(0)),
    },
    |n| Color::HSV360(
      *n.get(0).unwrap_or(&0),
      *n.get(1).unwrap_or(&0),
      *n.get(2).unwrap_or(&0)
    )
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  use jomini::TextTape;
  use rstest::rstest;

  #[rstest]
  #[case("c = { 0 0 0 }", Color::RGB(0, 0, 0))]
  #[case("c = { 12 23 244 }", Color::RGB(12, 23, 244))]
  #[case("c = rgb { 12 23 244 }", Color::RGB(12, 23, 244))]
  #[case("c = hsv { 0.1 0.5 0.8 }", Color::HSV(0.1, 0.5, 0.8))]
  #[case("c = hsv360 { 101 50 355 }", Color::HSV360(101, 50, 355))]
  fn read_color_with_valid_inputs_should_succeed(
    #[case] input: &str,
    #[case] expected: Color
  ) {
    // Arrange
    let tape = TextTape::from_slice(input.as_bytes()).unwrap();
    let reader = tape.windows1252_reader();
    let (_, _, color) = reader.fields().next().unwrap();
    
    // Act
    let color = color.read_color().unwrap();

    // Assert
    assert_eq!(color, expected);
  }
}
use chrono::NaiveDate;

pub struct Format {}

impl Format {
  pub fn is_space(c: char) -> bool {
    return c == ' ' || (c >= '\t' && c <= '\r');
  }

  pub fn is_not_space(c: char) -> bool {
    return !Format::is_space(c);
  }

  pub fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
  }

  pub fn string_to_date(date: String) -> Option<NaiveDate> {
    let segments: Vec<&str> = date.split('.').collect();
    let mut date = [1, 1, 1, 0, 0, 0];
    
    for i in 0..segments.len() {
      let segment = segments[i];
      let value = segment.parse::<u32>();
      match value {
        Ok(v) => date[i] = v,
        Err(_) => return None
      }
    }
    
    let [y, m, d, hh, mm, s] = date;
    let y_signed: i32 = y.try_into().unwrap_or_default();
    let first_of_month = NaiveDate::from_ymd_opt(y_signed, m, 1);

    let next_month_year = match m { 12 => y_signed + 1, _ => y_signed };
    let next_month_month = (m + 1) % 12;
    let next_month = NaiveDate::from_ymd_opt(next_month_year, next_month_month, 1);

    let max_day = match (first_of_month, next_month) {
      (Some(start), Some(end)) => end.signed_duration_since(start).num_days(),
      _ => return None
    }.try_into().ok().unwrap();

    if (1..=9999).contains(&y) &&
      (1..=12).contains(&m) &&
      (1..=max_day).contains(&d) &&
      (0..=23).contains(&hh) &&
      (0..=59).contains(&mm) &&
      (0..=59).contains(&s) 
    {
      NaiveDate::from_ymd_opt(y_signed, m, d)
    } else {
      None
    }
  }
}
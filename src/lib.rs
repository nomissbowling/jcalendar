#![doc(html_root_url = "https://docs.rs/jcalendar/0.1.2")]
//! Japanese Calendar for Rust
//!

pub mod cal;

/// tests
#[cfg(test)]
mod tests {
  use super::cal::{Cal, Term};
  use koyomi::Date;

  /// [-- --nocapture] [-- --show-output]
  #[test]
  fn jcalendar_tests() {
    let cal = Cal::new(vec![
      (0x20, 0xC0, 0xF0), // 月-金
      (0xF0, 0xC0, 0x20), // 土
      (0xC0, 0x00, 0x00), // 日
      (0x00, 0xFF, 0x00)]).unwrap(); // 祝
    cal.show_mat(Term::new().unwrap(), 11, true).unwrap();
    cal.show_list(Term::new().unwrap()).unwrap();
    cal.show_mat(Term{
      s: Date::parse("2023-10-29").expect("s"),
      e: Date::from_ymd(2023, 12, 2).expect("e")
    }, 11, true).unwrap();
    cal.show_mat(Term{
      s: Date::parse("2023-10-29").expect("s"),
      e: Date::from_ymd(2023, 12, 2).expect("e")
    }, 11, false).unwrap();
    assert_eq!(true, true);
  }
}

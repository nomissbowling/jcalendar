//! cal.rs
//!
//! calendar
//!

use std::error::Error;
use std::collections::HashMap;

use koyomi::{Calendar, Date, num_days, Weekday};
use chrono::{Local, Datelike};
use colored::{Colorize, Color};
use jput::{jput_pad, jputc_colored, jputs_colored, jprint_colored};
use jput::{jputs, jprint}; // jputc

/// term
pub struct Term {
  /// start
  pub s: Date,
  /// end
  pub e: Date
}

/// term
impl Term {
  /// construct
  pub fn new() -> Result<Term, Box<dyn Error>> {
    let err = "error Date::parse";
    let now = Local::now();
    let (y, m) = (now.year(), now.month());
    Ok(Term{
      s: Date::parse(&format!("{}-{}-01", y, m)).expect(err),
      e: Date::parse(&format!("{}-{}-{}", y, m, num_days(y, m))).expect(err)})
  }
}

/// holiday week name
pub fn holiday_week_name(d: &Date) -> (Weekday, char, String) {
  let w = d.weekday().clone();
  if let Some(hol) = d.holiday() {
    (w, '祝', hol) // must check drifting
  } else {
    let wj = w.japanese();
    let wjs = format!("{}", wj);
    match wj {
    '土' | '日' => (w, wj, wjs),
    _ => (w, wj, wjs) // now same as above
    }
  }
}

/// regular
pub fn regular(trim: i32, col_tbl: &HashMap<char, Color>,
  wcs: (Weekday, char, String)) -> (u8, Color, String) {
  let m = (trim / 2) as usize * 3; // trim / 2 * size::<utf-8-j>
  let u = wcs.2;
  let s = match wcs.1 {
  '祝' => {
    if trim >= 0 && u.len() > m { u.split_at(m).0.to_string() } else { u }
  }
  _ => "".to_string()
  };
  ((wcs.0 as u8 + 1) % 7, col_tbl[&wcs.1], s)
  // (wcs.0.num_days_from_sunday() as u8, col_tbl[&wcs.1], s)
}

/// cal
pub struct Cal {
  /// wn
  pub wn: Vec<char>,
  /// cols
  pub cols: Vec<Color>,
  /// color table
  pub col_tbl: HashMap<char, Color>
}

/// cal
impl Cal {
  /// construct
  pub fn new(pal: Vec<(u8, u8, u8)>) -> Result<Cal, Box<dyn Error>> {
    let wn = vec!['日', '月', '火', '水', '木', '金', '土'];
    let cols: Vec<Color> = pal.into_iter().map(|c|
      Color::TrueColor{r: c.0, g: c.1, b: c.2}).collect();
    let mut col_tbl: HashMap<char, Color> = wn.iter().map(|&w|
      (w, cols[0])).collect();
    col_tbl.get_mut(&'土').map(|v| *v = cols[1]);
    col_tbl.get_mut(&'日').map(|v| *v = cols[2]);
    col_tbl.insert('祝', cols[3]);

    Ok(Cal{wn, cols, col_tbl})
  }

  /// show_mat
  pub fn show_mat(&self, term: Term, cs: i32) -> Result<(), Box<dyn Error>> {
    let vd = Calendar::new(term.s, term.e).expect("error Calendar").make();
    jprint_colored!(self.cols[0], 7, "{}-{:02}\n",
      vd[0].year(), vd[0].month())?;
    for &n in &self.wn { jputc_colored(n, -cs, self.col_tbl[&n])?; }
    jprint!(0, "\n")?;
    let mut first = true;
    for ref d in &vd { // &Vec<Koyomi::Date>
      let (w, c, s) = regular(cs - 3, &self.col_tbl, holiday_week_name(d));
      if first { first = false; for _ in 0..w { jput_pad(true, cs, 0)?; } }
      jputs_colored(&format!("{:02}{}", d.day(), s).color(c), -cs)?;
      if w == 6 { jprint!(0, "\n")?; }
    }
    jprint!(0, "\n")?;
    Ok(())
  }

  /// show_list
  pub fn show_list(&self, term: Term) -> Result<(), Box<dyn Error>> {
    let vd = Calendar::new(term.s, term.e).expect("error Calendar").make();
    for ref d in &vd { // &Vec<Koyomi::Date>
      let (w, c, s) = regular(-1, &self.col_tbl, holiday_week_name(d));
      jprint_colored!(c, -16, "{} {} {} {}\n", d, self.wn[w as usize], w, s)?;
    }
    Ok(())
  }
}

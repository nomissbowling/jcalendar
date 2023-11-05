jcalendar
=========

Japanese Calendar for Rust

- [https://crates.io/crates/jcalendar]( https://crates.io/crates/jcalendar )


Samples
-------

- setup Cal with base 4 colors

```rust
let cal = Cal::new(vec![
  (0x20, 0xC0, 0xF0), // 月-金
  (0xF0, 0xC0, 0x20), // 土
  (0xC0, 0x00, 0x00), // 日
  (0x00, 0xFF, 0x00)]).unwrap(); // 祝
```

- list days of current month

```rust
cal.show_list(Term::new().unwrap()).unwrap();
```

```
2023-11-01 水 3
2023-11-02 木 4
2023-11-03 金 5 文化の日
2023-11-04 土 6
2023-11-05 日 0
...
2023-11-29 水 3
2023-11-30 木 4
```

- calender of current month (column width: 11, separate months: true)

```rust
cal.show_mat(Term::new().unwrap(), 11, true).unwrap();
```

```
2023-11
日         月         火         水         木         金         土
                                 01         02         03文化の日 04
05         06         07         08         09         10         11
12         13         14         15         16         17         18
19         20         21         22         23勤労感謝 24         25
26         27         28         29         30
```

- calender any term (column width: 11, separate months: true)

```rust
cal.show_mat(Term{
  s: Date::parse("2023-10-29").expect("s"),
  e: Date::from_ymd(2023, 12, 2).expect("e")
}, 11, true).unwrap();
```

```
2023-10
日         月         火         水         木         金         土
29         30         31
2023-11
日         月         火         水         木         金         土
                                 01         02         03文化の日 04
05         06         07         08         09         10         11
12         13         14         15         16         17         18
19         20         21         22         23勤労感謝 24         25
26         27         28         29         30
2023-12
日         月         火         水         木         金         土
                                                       01         02
```

- calender any term (column width: 11, separate months: false)

```rust
cal.show_mat(Term{
  s: Date::parse("2023-10-29").expect("s"),
  e: Date::from_ymd(2023, 12, 2).expect("e")
}, 11, false).unwrap();
```

```
2023-10
日         月         火         水         木         金         土
29         30         31         01         02         03文化の日 04
05         06         07         08         09         10         11
12         13         14         15         16         17         18
19         20         21         22         23勤労感謝 24         25
26         27         28         29         30         01         02
```


License
-------

MIT License

## ByteSize

[![CI](https://github.com/hyunsik/bytesize/actions/workflows/ci.yml/badge.svg)](https://github.com/hyunsik/bytesize/actions/workflows/ci.yml)
[![Crates.io Version](https://img.shields.io/crates/v/bytesize.svg)](https://crates.io/crates/bytesize)

`ByteSize` is a utility for human-readable byte count representations.

Features:

- Pre-defined constants for various size units (e.g., B, KB, KiB, MB, MiB, GB, GiB, ... PiB).
- `ByteSize` type which presents size units convertible to different size units.
- Arithmetic operations for `ByteSize`.
- FromStr impl for `ByteSize`, allowing to parse from string size representations like 1.5KiB and 521TiB.
- Serde support for binary and human-readable deserializers like JSON.

[API Documentation](https://docs.rs/bytesize)

## Example

### Human readable representations (SI unit and Binary unit)

```rust
fn assert_display(expected: &str, b: ByteSize) {
    assert_eq!(expected, format!("{}", b));
}

#[test]
fn test_display() {
    assert_display("215 B", ByteSize::b(215));
    assert_display("1.0 KiB", ByteSize::kib(1));
    assert_display("301.0 KiB", ByteSize::kib(301));
    assert_display("419.0 MiB", ByteSize::mib(419));
    assert_display("518.0 GiB", ByteSize::gib(518));
    assert_display("815.0 TiB", ByteSize::tib(815));
    assert_display("609.0 PiB", ByteSize::pib(609));
}

#[test]
fn test_display_alignment() {
    assert_eq!("|357 B     |", format!("|{:10}|", ByteSize(357)));
    assert_eq!("|     357 B|", format!("|{:>10}|", ByteSize(357)));
    assert_eq!("|357 B     |", format!("|{:<10}|", ByteSize(357)));
    assert_eq!("|  357 B   |", format!("|{:^10}|", ByteSize(357)));

    assert_eq!("|-----357 B|", format!("|{:->10}|", ByteSize(357)));
    assert_eq!("|357 B-----|", format!("|{:-<10}|", ByteSize(357)));
    assert_eq!("|--357 B---|", format!("|{:-^10}|", ByteSize(357)));
}

fn assert_to_string(expected: &str, b: ByteSize, si: bool) {
    assert_eq!(expected.to_string(), b.to_string_as(si));
}

#[test]
fn test_to_string_as() {
    assert_to_string("215 B", ByteSize::b(215), true);
    assert_to_string("215 B", ByteSize::b(215), false);
    assert_to_string("215 B", ByteSize::b(215), true);

    assert_to_string("1.0 KiB", ByteSize::kib(1), true);
    assert_to_string("1.0 KB", ByteSize::kib(1), false);

    assert_to_string("293.9 KiB", ByteSize::kb(301), true);
    assert_to_string("301.0 KB", ByteSize::kb(301), false);

    assert_to_string("1.0 MiB", ByteSize::mib(1), false);
    assert_to_string("1048.6 kB", ByteSize::mib(1), true);

    assert_to_string("1.9 GiB", ByteSize::mib(1907), true);
    assert_to_string("2.0 GB", ByteSize::mib(1908), false);

    assert_to_string("399.6 MiB", ByteSize::mb(419), true);
    assert_to_string("419.0 MB", ByteSize::mb(419), false);

    assert_to_string("399.6 MiB", ByteSize::mb(419), false);
    assert_to_string("419.0 MB", ByteSize::mb(419), true);

    assert_to_string("482.4 GiB", ByteSize::gb(518), false);
    assert_to_string("518.0 GB", ByteSize::gb(518), true);

    assert_to_string("741.2 TiB", ByteSize::tb(815), false);
    assert_to_string("815.0 TB", ByteSize::tb(815), true);

    assert_to_string("540.9 PiB", ByteSize::pb(609), false);
    assert_to_string("609.0 PB", ByteSize::pb(609), true);
}

#[test]
fn test_parsing_from_str() {
    // shortcut for writing test cases
    fn parse(s: &str) -> u64 {
        s.parse::<ByteSize>().unwrap().0
    }

    assert_to_string("540.9 PiB", ByteSize::pb(609), false);
    assert_to_string("609.0 PB", ByteSize::pb(609), true);
}
```

### Arithmetic operations

```rust
use bytesize::ByteSize;

fn byte_arithmetic_operator() {
    let x = ByteSize::mb(1);
    let y = ByteSize::kb(100);

    let plus = x + y;
    println!("{}", plus);

    let minus = ByteSize::tb(100) + ByteSize::gb(4);
    println!("{}", minus);
}
```

## ByteSize

[![Rust](https://github.com/foyer-rs/bytesize/actions/workflows/rust.yml/badge.svg)](https://github.com/foyer-rs/bytesize/actions/workflows/rust.yml)
[![Crates.io Version](https://img.shields.io/crates/v/foyer-bytesize.svg)](https://crates.io/crates/foyer-bytesize)

Forked from https://github.com/hyunsik/bytesize .

ByteSize is an utility for human-readable byte count representation.

Features:
* Pre-defined constants for various size units (e.g., B, Kb, Kib, Mb, Mib, Gb, Gib, ... PB)
* `ByteSize` type which presents size units convertible to different size units.
* Artimetic operations for `ByteSize`
* FromStr impl for `ByteSize`, allowing to parse from string size representations like 1.5KiB and 521TiB.
* Serde support for binary and human-readable deserializers like JSON

[API Documentation](https://docs.rs/foyer-bytesize/)

### Differences from the original `bytesize`

- Use SI format by default with `Display`.
- Use "KiB" for SI unit.

Considering the changes, the version of `foyer-bytesize` crate starts from "2" to differ from the original `bytesize` crate.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
bytesize = { package = "foyer-bytesize", version = "2", features = ["serde"]}
```

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

        assert_to_string("1.0 KiB", ByteSize::kib(1), true);
        assert_to_string("1.0 KB", ByteSize::kib(1), false);

        assert_to_string("293.9 KiB", ByteSize::kb(301), true);
        assert_to_string("301.0 KB", ByteSize::kb(301), false);

        assert_to_string("1.0 MiB", ByteSize::mib(1), true);
        assert_to_string("1048.6 KB", ByteSize::mib(1), false);

        // a bug case: https://github.com/flang-project/bytesize/issues/8
        assert_to_string("1.9 GiB", ByteSize::mib(1907), true);
        assert_to_string("2.0 GB", ByteSize::mib(1908), false);

        assert_to_string("399.6 MiB", ByteSize::mb(419), true);
        assert_to_string("419.0 MB", ByteSize::mb(419), false);

        assert_to_string("482.4 GiB", ByteSize::gb(518), true);
        assert_to_string("518.0 GB", ByteSize::gb(518), false);

        assert_to_string("741.2 TiB", ByteSize::tb(815), true);
        assert_to_string("815.0 TB", ByteSize::tb(815), false);

        assert_to_string("540.9 PiB", ByteSize::pb(609), true);
        assert_to_string("609.0 PB", ByteSize::pb(609), false);
    }
```

### Arithmetic operations
```rust
use bytesize::ByteSize;

fn byte_arithmetic_operator() {
  let x = ByteSize::mb(1);
  let y = ByteSize::kb(100);

  let plus = x + y;
  print!("{}", plus);

  let minus = ByteSize::tb(100) + ByteSize::gb(4);
  print!("{}", minus);
}
```

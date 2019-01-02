## ByteSize

[![Build Status](https://travis-ci.org/hyunsik/bytesize.svg?branch=master)](https://travis-ci.org/hyunsik/bytesize)
[![Crates.io Version](https://img.shields.io/crates/v/bytesize.svg)](https://crates.io/crates/bytesize)

ByteSize is a utility for human-readable byte count representation.

[API Documentation](https://docs.rs/bytesize/)

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
bytesize = "1.0.0"
```

## Examples

### Human readable representations (SI unit and Binary unit)

```rust
extern crate bytesize;

use bytesize::{ByteSize, IEC SI};

assert_eq!("482.4 GiB".to_string(), ByteSize::gb(518).humanize(IEC));
assert_eq!("518.0 GB".to_string(), ByteSize::gb(518).humanize(SI));
```

### Arithmetic operations

```rust
extern crate bytesize;

use bytesize::ByteSize;

let x = ByteSize::mb(1);
let y = ByteSize::kb(100);

let sum = x + y;
assert_eq!(sum, ByteSize::kb(1100));

let product = 10u32 * x;
assert_eq!(product, ByteSize::mb(10));
```

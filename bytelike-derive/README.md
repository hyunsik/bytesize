## ByteLike Derive

[![CI](https://github.com/hyunsik/bytesize/actions/workflows/ci.yml/badge.svg)](https://github.com/hyunsik/bytesize/actions/workflows/ci.yml)
[![Crates.io Version](https://img.shields.io/crates/v/bytesize.svg)](https://crates.io/crates/bytesize)

ByteLike Derive is a procedural macro crate for deriving `ByteLike` functions for deriving byte-like new types.

It's a procedural macro that was created based off the implementation of [Bytesize](https://crates.io/crates/bytesize).

## Usage 

Add this to your `Cargo.toml`

```toml
[dependencies]
bytelike = { version = "0.1" }
bytelike-derive = { version = "0.1", features = ["serde"] }
serde = { version = "1.0" }
serde_derive = { version = "1.0" }
```

Or if don't want serde:
```toml
[dependencies]
bytelike = { version = "0.1" }
bytelike-derive = { version = "0.1" }
```

Next, define your new type and derive `ByteLike` for it:

```rust
use bytelike_derive::ByteLike;

#[derive(ByteLike)]
pub struct NewType(pub u64);
```

Now you can do lots of useful byte-like things with your new type:
```rust
let new_type: NewType = "5KiB".parse().unwrap();
let other_type: NewType = NewType::kib(5);
let sum = new_type + other_type;
```
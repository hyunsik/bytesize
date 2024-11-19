## ByteLike-Derive

[![CI](https://github.com/hyunsik/bytesize/actions/workflows/ci.yml/badge.svg)](https://github.com/hyunsik/bytesize/actions/workflows/ci.yml)
[![Crates.io Version](https://img.shields.io/crates/v/bytelike-derive.svg)](https://crates.io/crates/bytelike-derive)

ByteLike Derive is a procedural macro crate for deriving `ByteLike` functions for deriving byte-like new types.

It's a procedural macro that was created based off the implementation of [ByteSize](https://crates.io/crates/bytesize).

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
let new_type_a: NewType = "5KiB".parse().unwrap();
let new_type_b: NewType = NewType::kib(5);
let sum = new_type + other_type;
let sum = new_type + 5;
```
See the documentation for ByteSize to see more examples of what you can do with the ByteLike new type (just replace 
ByteSize with your new type name).

In addition, if you only want some of the ByteLike functions derived, you can use any of the following derives in an 
additive manner:
* ByteLikeConstructor
* ByteLikeDisplay
* ByteLikeParse
* ByteLikeOps
* ByteLikeFromStr
* ByteLikeSerde (requires the `serde` feature)

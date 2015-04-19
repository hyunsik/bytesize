ByteSize is an utility that easily makes bytes size representation 
and helps its arithmetic operations.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
bytesize = "0.0.1"
```

and this to your crate root:
```rust
extern crate bytesize;
```

## Example

```rust
extern crate bytesize;

use bytesize::ByteSize;
 
fn byte_arithmetic_operator() {
  let x = ByteSize::mb(1);
  let y = ByteSize::kb(100);
   
  let plus = x + y;
  print!("{} bytes", plus.as_usize());
   
  let minus = ByteSize::tb(100) - ByteSize::gb(4);
  print!("{} bytes", minus.as_usize());
}
 ```

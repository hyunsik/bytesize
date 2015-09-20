//! ByteSize is an utility that easily makes bytes size representation
//! and helps its arithmetic operations.
//!
//! ## Example
//!
//! ```ignore
//! extern crate bytesize;
//!
//! use bytesize::ByteSize;
//!
//! fn byte_arithmetic_operator() {
//!   let x = ByteSize::mb(1);
//!   let y = ByteSize::kb(100);
//!
//!   let plus = x + y;
//!   print!("{} bytes", plus.as_usize());
//!
//!   let minus = ByteSize::tb(100) - ByteSize::gb(4);
//!   print!("{} bytes", minus.as_usize());
//! }
//! ```

extern crate num;

use std::fmt::{Display,Formatter,Result};
use std::ops::{Add,Sub,Mul,Div};


/// byte size for 1 byte
pub static B: usize = 1;
/// bytes size for 1 kilobyte
pub static KB: usize = 1000;
/// bytes size for 1 megabyte
pub static MB: usize = 1000000;
/// bytes size for 1 gigabyte
pub static GB: usize = 1000000000;
/// bytes size for 1 terabyte
pub static TB: usize = 1000000000000;
/// bytes size for 1 petabyte
pub static PB: usize = 1000000000000000;

/// bytes size for 1 kibibyte
pub static KIB: usize = 1024;
/// bytes size for 1 mebibyte
pub static MIB: usize = 1048576;
/// bytes size for 1 gibibyte
pub static GIB: usize = 1073741824;
/// bytes size for 1 tebibyte
pub static TIB: usize = 1099511627776;
/// bytes size for 1 pebibyte
pub static PIB: usize = 1125899906842624;

#[derive(Debug, Copy, Clone)]
/// Byte size representation
pub struct ByteSize {
  size: usize
}

impl ByteSize {
  #[inline(always)]
  pub fn b(size: usize) -> ByteSize {
    ByteSize {size: size}
  }

  #[inline(always)]
  pub fn kb(size: usize) -> ByteSize {
    ByteSize {size: size * KB}
  }

  #[inline(always)]
  pub fn kib(size: usize) -> ByteSize {
   ByteSize {size: size * KIB}
  }

  #[inline(always)]
  pub fn mb(size: usize) -> ByteSize {
    ByteSize {size: size * MB}
  }

  #[inline(always)]
  pub fn mib(size: usize) -> ByteSize {
    ByteSize {size: size * MIB}
  }

  #[inline(always)]
  pub fn gb(size: usize) -> ByteSize {
    ByteSize {size: size * GB}
  }

  #[inline(always)]
  pub fn gib(size: usize) -> ByteSize {
    ByteSize {size: size * GIB}
  }

  #[inline(always)]
  pub fn tb(size: usize) -> ByteSize {
    ByteSize {size: size * TB}
  }

  #[inline(always)]
  pub fn tib(size: usize) -> ByteSize {
    ByteSize {size: size * TIB}
  }

  #[inline(always)]
  pub fn pb(size: usize) -> ByteSize {
    ByteSize {size: size * PB}
  }

  #[inline(always)]
  pub fn pib(size: usize) -> ByteSize {
    ByteSize {size: size * PIB}
  }

  #[inline(always)]
  pub fn as_usize(&self) -> usize {
    self.size
  }
}

static UNITS: &'static str = "KMGTPE";

impl Display for ByteSize {
  fn fmt(&self, f: &mut Formatter) -> Result {
    match self.size {
      sz if sz < KB => {write!(f, "{} B", sz)},
      _ => {
          let exp = ((self.size as f64).ln() / (KB as f64).ln()) as usize;
          write!(f, "{} {}B",
            (self.size / num::pow(KB, exp)),
            UNITS.as_bytes()[exp - 1] as char)
      }
    }
  }
}

impl Add<usize> for ByteSize {
  type Output = ByteSize;

  #[inline(always)]
  fn add(self, rhs: usize) -> ByteSize {
    ByteSize {size: (self.size + rhs)}
  }
}

impl Add<ByteSize> for ByteSize {
  type Output = ByteSize;

  #[inline(always)]
  fn add(self, rhs: ByteSize) -> ByteSize {
    ByteSize {size: (self.size + rhs.size)}
  }
}

impl Sub<usize> for ByteSize {
  type Output = ByteSize;

  #[inline(always)]
  fn sub(self, rhs: usize) -> ByteSize {
    ByteSize {size: (self.size - rhs)}
  }
}

impl Sub<ByteSize> for ByteSize {
  type Output = ByteSize;

  #[inline(always)]
  fn sub(self, rhs: ByteSize) -> ByteSize {
    ByteSize {size: (self.size - rhs.size)}
  }
}

impl Mul<usize> for ByteSize {
  type Output = ByteSize;

  #[inline(always)]
  fn mul(self, rhs: usize) -> ByteSize {
    ByteSize {size: (self.size * rhs)}
  }
}

impl Div<usize> for ByteSize {
  type Output = ByteSize;

  #[inline(always)]
  fn div(self, rhs: usize) -> ByteSize {
    ByteSize {size: (self.size / rhs)}
  }
}

#[test]
fn test_arithmetic() {
  let x = ByteSize::mb(1);
  let y = ByteSize::kb(100);

  assert_eq!(
    (x + y).as_usize(),
    1100000
  );
  assert_eq!(
    (x + (100*1000)).as_usize(),
    1100000
  );

  assert_eq!(
    (x - y).as_usize(),
    900000
  );
  assert_eq!(
    (x - (100*1000)).as_usize(),
    900000
  );
}

#[allow(dead_code)]
fn assert_display(expected: &str, b: ByteSize) {
  assert_eq!(expected, format!("{}", b));
}

#[test]
fn test_display() {
  assert_display("100 KB", ByteSize::kb(100));
  assert_display("128 KB", ByteSize::kb(128));

  assert_display("804 B", ByteSize::b(804));
}

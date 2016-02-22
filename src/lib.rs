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
//!
//! It also provides its human readable string as follows:
//!
//! ```ignore
//!  assert_eq!("482 GiB".to_string(), ByteSize::gb(518).to_string(true));
//!  assert_eq!("518 GB".to_string(), ByteSize::gb(518).to_string(false));
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

  pub fn to_string(&self, si: bool) -> String
  {

    let unit = if si { KIB } else { KB };

    if self.size < unit {
      {format!("{} B", self.size)}
    } else {
      let mut exp = ((self.size as f64).ln() / (if si {LN_KIB} else {LN_KB})) as usize;
      if exp == 0 {
        exp = 1;
      }

      if si {
        format!("{} {}iB",(self.size / num::pow(unit, exp)), UNITS_SI.as_bytes()[exp - 1] as char)
      } else {
        format!("{} {}B",(self.size / num::pow(unit, exp)), UNITS.as_bytes()[exp - 1] as char)
      }
    }
  }
}

static UNITS:    &'static str = "KMGTPE";
static UNITS_SI: &'static str = "kMGTPE";
static LN_KB: f64 = 6.931471806; // ln 1024
static LN_KIB: f64 = 6.907755279; // ln 1000

impl Display for ByteSize {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.to_string(false))
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


#[cfg(test)]
mod tests {
  use super::*;

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

  fn assert_display(expected: &str, b: ByteSize) {
    assert_eq!(expected, format!("{}", b));
  }

  #[test]
  fn test_display() {
    assert_display("215 B", ByteSize::b(215));
    assert_display("1 KB", ByteSize::kb(1));
    assert_display("301 KB", ByteSize::kb(301));
    assert_display("419 MB", ByteSize::mb(419));
    assert_display("518 GB", ByteSize::gb(518));
    assert_display("815 TB", ByteSize::tb(815));
    assert_display("609 PB", ByteSize::pb(609));
  }

  fn assert_to_string(expected: &str, b: ByteSize, si: bool) {
    assert_eq!(expected.to_string(), b.to_string(si));
  }

  #[test]
  fn test_to_string() {
    assert_to_string("215 B", ByteSize::b(215), true);
    assert_to_string("215 B", ByteSize::b(215), false);

    assert_to_string("1 kiB", ByteSize::kib(1), true);
    assert_to_string("1 KB", ByteSize::kib(1), false);

    assert_to_string("293 kiB", ByteSize::kb(301), true);
    assert_to_string("301 KB", ByteSize::kb(301), false);

    assert_to_string("1 MiB", ByteSize::mib(1), true);
    assert_to_string("1048 KB", ByteSize::mib(1), false);

    assert_to_string("399 MiB", ByteSize::mb(419), true);
    assert_to_string("419 MB", ByteSize::mb(419), false);

    assert_to_string("482 GiB", ByteSize::gb(518), true);
    assert_to_string("518 GB", ByteSize::gb(518), false);

    assert_to_string("741 TiB", ByteSize::tb(815), true);
    assert_to_string("815 TB", ByteSize::tb(815), false);

    assert_to_string("540 PiB", ByteSize::pb(609), true);
    assert_to_string("609 PB", ByteSize::pb(609), false);
  }
}

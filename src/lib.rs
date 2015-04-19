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


use std::ops::{Add,Sub,Mul,Div};

/// byte size for 1 byte
pub static B: u64 = 1;
/// bytes size for 1 kilobyte
pub static KB: u64 = 1000;
/// bytes size for 1 megabyte
pub static MB: u64 = 1000000;
/// bytes size for 1 gigabyte
pub static GB: u64 = 1000000000;
/// bytes size for 1 terabyte
pub static TB: u64 = 1000000000000;
/// bytes size for 1 petabyte
pub static PB: u64 = 1000000000000000;

/// bytes size for 1 kibibyte
pub static KIB: u64 = 1024;
/// bytes size for 1 mebibyte
pub static MIB: u64 = 1048576;
/// bytes size for 1 gibibyte
pub static GIB: u64 = 1073741824;
/// bytes size for 1 tebibyte
pub static TIB: u64 = 1099511627776;
/// bytes size for 1 pebibyte
pub static PIB: u64 = 1125899906842624;

#[derive(Debug, Copy, Clone)]
/// Byte size representation
pub struct ByteSize {
  size: u64
}

impl ByteSize {
  #[inline(always)]
  pub fn b(size: u64) -> ByteSize {
    ByteSize {size: size}
  }

  #[inline(always)]
  pub fn kb(size: u64) -> ByteSize {
    ByteSize {size: size * KB}
  }

  #[inline(always)]
  pub fn kib(size: u64) -> ByteSize {
   ByteSize {size: size * KIB} 
  }

  #[inline(always)]
  pub fn mb(size: u64) -> ByteSize {
    ByteSize {size: size * MB}
  }

  #[inline(always)]
  pub fn mib(size: u64) -> ByteSize {
    ByteSize {size: size * MIB}
  }

  #[inline(always)]
  pub fn gb(size: u64) -> ByteSize {
    ByteSize {size: size * GB}
  }

  #[inline(always)]
  pub fn gib(size: u64) -> ByteSize {
    ByteSize {size: size * GIB}
  }

  #[inline(always)]
  pub fn tb(size: u64) -> ByteSize {
    ByteSize {size: size * TB}
  }

  #[inline(always)]
  pub fn tib(size: u64) -> ByteSize {
    ByteSize {size: size * TIB}
  }

  #[inline(always)]
  pub fn pb(size: u64) -> ByteSize {
    ByteSize {size: size * PB}
  }

  #[inline(always)]
  pub fn pib(size: u64) -> ByteSize {
    ByteSize {size: size * PIB}
  }

  #[inline(always)]
  pub fn as_u64(&self) -> u64 {
    self.size
  }

  #[inline(always)]
  pub fn as_usize(&self) -> usize {
    self.size as usize
  }
}

impl Add<u64> for ByteSize {
  type Output = ByteSize;

  #[inline(always)]
  fn add(self, rhs: u64) -> ByteSize {
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

impl Sub<u64> for ByteSize {
  type Output = ByteSize;

  #[inline(always)]
  fn sub(self, rhs: u64) -> ByteSize {
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

impl Mul<u64> for ByteSize {
  type Output = ByteSize;

  #[inline(always)]
  fn mul(self, rhs: u64) -> ByteSize {
    ByteSize {size: (self.size * rhs)}
  }
}

impl Div<u64> for ByteSize {
  type Output = ByteSize;

  #[inline(always)]
  fn div(self, rhs: u64) -> ByteSize {
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
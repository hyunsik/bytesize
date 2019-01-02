//! ByteSize is a utility that easily makes bytes size representation and helps
//! its arithmetic operations.
//!
//! ## Human Readable Representation
//!
//! ByteSize provides a human readable string conversion as follows:
//!
//! ```
//! extern crate bytesize;
//!
//! use bytesize::{ByteSize, IEC, SI};
//!
//! assert_eq!("482.4 GiB".to_string(), ByteSize::gb(518).humanize(IEC));
//! assert_eq!("518.0 GB".to_string(), ByteSize::gb(518).humanize(SI));
//! ```
//!
//! ## Arithmetic
//!
//! ```
//! extern crate bytesize;
//!
//! use bytesize::ByteSize;
//!
//! let x = ByteSize::mb(1);
//! let y = ByteSize::kb(100);
//!
//! let sum = x + y;
//! assert_eq!(sum, ByteSize::kb(1100));
//!
//! let product = 10u32 * x;
//! assert_eq!(product, ByteSize::mb(10));
//! ```

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::{Add, Mul};

/// byte size for 1 byte
pub const B: u64 = 1;
/// bytes size for 1 kilobyte
pub const KB: u64 = 1_000;
/// bytes size for 1 megabyte
pub const MB: u64 = 1_000_000;
/// bytes size for 1 gigabyte
pub const GB: u64 = 1_000_000_000;
/// bytes size for 1 terabyte
pub const TB: u64 = 1_000_000_000_000;
/// bytes size for 1 petabyte
pub const PB: u64 = 1_000_000_000_000_000;

/// bytes size for 1 kibibyte
pub const KIB: u64 = 1_024;
/// bytes size for 1 mebibyte
pub const MIB: u64 = 1_048_576;
/// bytes size for 1 gibibyte
pub const GIB: u64 = 1_073_741_824;
/// bytes size for 1 tebibyte
pub const TIB: u64 = 1_099_511_627_776;
/// bytes size for 1 pebibyte
pub const PIB: u64 = 1_125_899_906_842_624;

static UNITS: &'static str = "KMGTPE";
static UNITS_SI: &'static str = "kMGTPE";
static LN_KB: f64 = 6.931471806; // ln 1024
static LN_KIB: f64 = 6.907755279; // ln 1000

pub fn kb<V: Into<u64>>(size: V) -> u64 {
    size.into() * KB
}

pub fn kib<V: Into<u64>>(size: V) -> u64 {
    size.into() * KIB
}

pub fn mb<V: Into<u64>>(size: V) -> u64 {
    size.into() * MB
}

pub fn mib<V: Into<u64>>(size: V) -> u64 {
    size.into() * MIB
}

pub fn gb<V: Into<u64>>(size: V) -> u64 {
    size.into() * GB
}

pub fn gib<V: Into<u64>>(size: V) -> u64 {
    size.into() * GIB
}

pub fn tb<V: Into<u64>>(size: V) -> u64 {
    size.into() * TB
}

pub fn tib<V: Into<u64>>(size: V) -> u64 {
    size.into() * TIB
}

pub fn pb<V: Into<u64>>(size: V) -> u64 {
    size.into() * PB
}

pub fn pib<V: Into<u64>>(size: V) -> u64 {
    size.into() * PIB
}

/// 32 * 1024 Byte = 32 KiB
pub struct IEC;

/// 32 * 1000 Byte = 32 KB
pub struct SI;

/// 32 * 1024 Byte = 32K
pub struct Sort;

fn humanize(bytes: u64, si: bool, prefix: &str, suffix: &str) -> String {
    let unit = if si { KB } else { KIB };
    let unit_base = if si { LN_KB } else { LN_KIB };
    let unit_prefix = if si {
        UNITS_SI.as_bytes()
    } else {
        UNITS.as_bytes()
    };

    if bytes < unit {
        format!("{}{}B", bytes, prefix)
    } else {
        let size = bytes as f64;
        let exp = match (size.ln() / unit_base) as usize {
            e if e == 0 => 1,
            e => e,
        };

        format!(
            "{:.1}{}{}{}",
            (size / unit.pow(exp as u32) as f64),
            prefix,
            unit_prefix[exp - 1] as char,
            suffix
        )
    }
}

pub trait ByteFormatter {
    fn humanize(&self, bytes: u64) -> String;
}

impl ByteFormatter for IEC {
    fn humanize(&self, bytes: u64) -> String {
        humanize(bytes, false, " ", "iB")
    }
}

impl ByteFormatter for SI {
    fn humanize(&self, bytes: u64) -> String {
        humanize(bytes, true, " ", "B")
    }
}

impl ByteFormatter for Sort {
    fn humanize(&self, bytes: u64) -> String {
        humanize(bytes, false, "", "")
    }
}

/// Byte size representation
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ByteSize(pub u64);

impl ByteSize {
    #[inline(always)]
    pub fn b(size: u64) -> ByteSize {
        ByteSize(size)
    }

    #[inline(always)]
    pub fn kb(size: u64) -> ByteSize {
        ByteSize(size * KB)
    }

    #[inline(always)]
    pub fn kib(size: u64) -> ByteSize {
        ByteSize(size * KIB)
    }

    #[inline(always)]
    pub fn mb(size: u64) -> ByteSize {
        ByteSize(size * MB)
    }

    #[inline(always)]
    pub fn mib(size: u64) -> ByteSize {
        ByteSize(size * MIB)
    }

    #[inline(always)]
    pub fn gb(size: u64) -> ByteSize {
        ByteSize(size * GB)
    }

    #[inline(always)]
    pub fn gib(size: u64) -> ByteSize {
        ByteSize(size * GIB)
    }

    #[inline(always)]
    pub fn tb(size: u64) -> ByteSize {
        ByteSize(size * TB)
    }

    #[inline(always)]
    pub fn tib(size: u64) -> ByteSize {
        ByteSize(size * TIB)
    }

    #[inline(always)]
    pub fn pb(size: u64) -> ByteSize {
        ByteSize(size * PB)
    }

    #[inline(always)]
    pub fn pib(size: u64) -> ByteSize {
        ByteSize(size * PIB)
    }

    #[inline(always)]
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    #[deprecated(since = "1.1.0", note = "use `bs.humanize(SI|IEC)`")]
    #[inline(always)]
    pub fn to_string_as(&self, si_unit: bool) -> String {
        if si_unit {
            self.humanize(SI)
        } else {
            self.humanize(IEC)
        }
    }

    /// Returns humanized String representation.
    ///
    /// ## Examples
    ///
    /// ```
    /// # extern crate bytesize;
    /// # use bytesize::*;
    /// assert_eq!("1.0 KiB", ByteSize::b(1024).humanize(IEC));
    /// assert_eq!("1.0 kB", ByteSize::b(1000).humanize(SI));
    /// ```
    #[inline(always)]
    pub fn humanize<F>(&self, fmt: F) -> String
    where
        F: ByteFormatter,
    {
        fmt.humanize(self.0)
    }
}

#[deprecated(since = "1.1.0", note = "use `ByteSize::b(bytes).humanize(SI|IEC)`")]
pub fn to_string(bytes: u64, si_prefix: bool) -> String {
    if si_prefix {
        humanize(bytes, si_prefix, " ", "B")
    } else {
        humanize(bytes, si_prefix, " ", "iB")
    }
}

impl Display for ByteSize {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.humanize(SI))
    }
}

impl Debug for ByteSize {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self)
    }
}

macro_rules! commutative_op {
    ($t:ty) => {
        impl Add<$t> for ByteSize {
            type Output = ByteSize;
            #[inline(always)]
            fn add(self, rhs: $t) -> ByteSize {
                ByteSize(self.0 + (rhs as u64))
            }
        }

        impl Add<ByteSize> for $t {
            type Output = ByteSize;
            #[inline(always)]
            fn add(self, rhs: ByteSize) -> ByteSize {
                ByteSize(rhs.0 + (self as u64))
            }
        }

        impl Mul<$t> for ByteSize {
            type Output = ByteSize;
            #[inline(always)]
            fn mul(self, rhs: $t) -> ByteSize {
                ByteSize(self.0 * (rhs as u64))
            }
        }

        impl Mul<ByteSize> for $t {
            type Output = ByteSize;
            #[inline(always)]
            fn mul(self, rhs: ByteSize) -> ByteSize {
                ByteSize(rhs.0 * (self as u64))
            }
        }
    };
}

commutative_op!(u64);
commutative_op!(u32);
commutative_op!(u16);
commutative_op!(u8);

impl Add<ByteSize> for ByteSize {
    type Output = ByteSize;

    #[inline(always)]
    fn add(self, rhs: ByteSize) -> ByteSize {
        ByteSize(self.0 + rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_op() {
        let x = ByteSize::mb(1);
        let y = ByteSize::kb(100);

        assert_eq!((x + y).as_u64(), 1_100_000u64);

        assert_eq!((x + (100 * 1000) as u64).as_u64(), 1_100_000);

        assert_eq!((x * 2u64).as_u64(), 2_000_000);
    }

    #[test]
    fn test_arithmetic_primitives() {
        let x = ByteSize::mb(1);

        assert_eq!((x + MB as u64).as_u64(), 2_000_000);

        assert_eq!((x + MB as u32).as_u64(), 2_000_000);

        assert_eq!((x + KB as u16).as_u64(), 1_001_000);

        assert_eq!((x + B as u8).as_u64(), 1_000_001);
    }

    #[test]
    fn test_comparison() {
        assert!(ByteSize::mb(1) == ByteSize::kb(1000));
        assert!(ByteSize::mib(1) == ByteSize::kib(1024));
        assert!(ByteSize::mb(1) != ByteSize::kib(1024));
        assert!(ByteSize::mb(1) < ByteSize::kib(1024));
        assert!(ByteSize::b(0) < ByteSize::tib(1));
    }

    fn assert_display(expected: &str, b: ByteSize) {
        assert_eq!(expected, format!("{}", b));
    }

    #[test]
    fn test_display() {
        assert_display("215 B", ByteSize::b(215));
        assert_display("1.0 kB", ByteSize::kb(1));
        assert_display("301.0 kB", ByteSize::kb(301));
        assert_display("419.0 MB", ByteSize::mb(419));
        assert_display("518.0 GB", ByteSize::gb(518));
        assert_display("815.0 TB", ByteSize::tb(815));
        assert_display("609.0 PB", ByteSize::pb(609));
    }

    fn assert_humanize<F>(expected: &str, b: ByteSize, fmt: F)
    where
        F: ByteFormatter,
    {
        assert_eq!(expected.to_string(), b.humanize(fmt));
    }

    #[test]
    fn test_humanize() {
        assert_humanize("215 B", ByteSize::b(215), IEC);
        assert_humanize("215 B", ByteSize::b(215), SI);

        assert_humanize("1.0 KiB", ByteSize::kib(1), IEC);
        assert_humanize("1.0 kB", ByteSize::kib(1), SI);
        assert_humanize("1.0K", ByteSize::kib(1), Sort);

        assert_humanize("293.9 KiB", ByteSize::kb(301), IEC);
        assert_humanize("301.0 kB", ByteSize::kb(301), SI);
        assert_humanize("293.9K", ByteSize::kb(301), Sort);

        assert_humanize("1.0 MiB", ByteSize::mib(1), IEC);
        assert_humanize("1048.6 kB", ByteSize::mib(1), SI);
        assert_humanize("1.0M", ByteSize::mib(1), Sort);

        // a bug case: https://github.com/flang-project/bytesize/issues/8
        assert_humanize("1.9 GiB", ByteSize::mib(1907), IEC);
        assert_humanize("2.0 GB", ByteSize::mib(1908), SI);
        assert_humanize("1.9G", ByteSize::mib(1907), Sort);

        assert_humanize("399.6 MiB", ByteSize::mb(419), IEC);
        assert_humanize("419.0 MB", ByteSize::mb(419), SI);
        assert_humanize("399.6M", ByteSize::mb(419), Sort);

        assert_humanize("482.4 GiB", ByteSize::gb(518), IEC);
        assert_humanize("518.0 GB", ByteSize::gb(518), SI);
        assert_humanize("482.4G", ByteSize::gb(518), Sort);

        assert_humanize("741.2 TiB", ByteSize::tb(815), IEC);
        assert_humanize("815.0 TB", ByteSize::tb(815), SI);
        assert_humanize("741.2T", ByteSize::tb(815), Sort);

        assert_humanize("540.9 PiB", ByteSize::pb(609), IEC);
        assert_humanize("609.0 PB", ByteSize::pb(609), SI);
        assert_humanize("540.9P", ByteSize::pb(609), Sort);
    }

    #[test]
    fn test_default() {
        assert_eq!(ByteSize::b(0), ByteSize::default());
    }

    #[test]
    fn test_to_string() {
        assert_humanize("609.0 PB", ByteSize::pb(609), SI);
    }
}

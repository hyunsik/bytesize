use bytelike_derive::*;
pub use bytelike::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default, ByteLike)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct ByteSize(pub u64);

#[cfg(test)]
mod tests {
    use super::*;
    use bytelike::{B, KB, MB}; // Import the constants we need

    #[test]
    fn test_arithmetic_op() {
        let mut x = ByteSize::mb(1);
        let y = ByteSize::kb(100);

        assert_eq!((x + y).as_u64(), 1_100_000u64);

        assert_eq!((x + (100 * 1000) as u64).as_u64(), 1_100_000);

        assert_eq!((x * 2u64).as_u64(), 2_000_000);

        x += y;
        assert_eq!(x.as_u64(), 1_100_000);
        x *= 2u64;
        assert_eq!(x.as_u64(), 2_200_000);
    }

    #[allow(clippy::unnecessary_cast)]
    #[test]
    fn test_arithmetic_primitives() {
        let mut x = ByteSize::mb(1);

        assert_eq!((x + MB as u64).as_u64(), 2_000_000);

        assert_eq!((x + MB as u32).as_u64(), 2_000_000);

        assert_eq!((x + KB as u16).as_u64(), 1_001_000);

        assert_eq!((x + B as u8).as_u64(), 1_000_001);

        x += MB as u64;
        x += MB as u32;
        x += 10u16;
        x += 1u8;
        assert_eq!(x.as_u64(), 3_000_011);
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

    #[test]
    fn test_default() {
        assert_eq!(ByteSize::b(0), ByteSize::default());
    }

    #[test]
    fn test_to_string() {
        assert_to_string("609.0 PB", ByteSize::pb(609), false);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        use serde_derive::{Deserialize, Serialize};
        use serde_json;
        use toml;

        #[derive(Serialize, Deserialize)]
        struct S {
            x: ByteSize,
        }

        let s: S = serde_json::from_str(r#"{ "x": "5 B" }"#).unwrap();
        assert_eq!(s.x, ByteSize(5));

        let s: S = serde_json::from_str(r#"{ "x": 1048576 }"#).unwrap();
        assert_eq!(s.x, "1 MiB".parse::<ByteSize>().unwrap());

        let s: S = toml::from_str(r#"x = "2.5 MiB""#).unwrap();
        assert_eq!(s.x, "2.5 MiB".parse::<ByteSize>().unwrap());

        // i64 MAX
        let s: S = toml::from_str(r#"x = "9223372036854775807""#).unwrap();
        assert_eq!(s.x, "9223372036854775807".parse::<ByteSize>().unwrap());
    }
}

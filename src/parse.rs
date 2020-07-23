use super::ByteSize;

impl std::str::FromStr for ByteSize {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.parse::<u64>() {
            Ok(v) => return Ok(Self(v)), // simple case, bytes
            Err(_) => {},
        };
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn when_ok() {
        // shortcut for writing test cases
        fn parse(s: &str) -> u64 {
            s.parse::<ByteSize>().unwrap().0
        }

        assert_eq!("0".parse::<ByteSize>().unwrap().0, 0);
        assert_eq!(parse("0"), 0);
        assert_eq!(parse("500"), 500);
        assert_eq!(parse("1K"), 1 * MB);
        assert_eq!(parse("1Ki"), 1 * KIB);
        assert_eq!(parse("1.5Ki"), (1.5 * KIB as f64) as u64);
        assert_eq!(parse("1KiB"), 1 * KIB);
        assert_eq!(parse("1.5KiB"), (1.5 * KIB as f64) as u64);
        assert_eq!(parse("3 MB"), 3 * MB);
        assert_eq!(parse("4 MiB"), 4 * MIB);
        assert_eq!(parse("6 GB"), 6 * GB);
        assert_eq!(parse("4 GiB"), 4 * GIB);
        assert_eq!(parse("88TB"), 88 * TB);
        assert_eq!(parse("521TiB"), 521 * TIB);
        assert_eq!(parse("8 PB"), 8 * PB);
        assert_eq!(parse("12 PB"), 12 * PIB);
    }
}

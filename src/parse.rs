use super::ByteSize;

impl std::str::FromStr for ByteSize {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.parse::<u64>() {
            Ok(v) => return Ok(Self(v)), // simple case, bytes
            Err(_) => {}
        };
        let number: String = value
            .chars()
            .take_while(|c| c.is_digit(10) || c == &'.')
            .collect();
        match number.parse::<f64>() {
            Ok(v) => {
                let suffix: String = value
                    .chars()
                    .skip_while(|c| c.is_whitespace() || c.is_digit(10) || c == &'.')
                    .collect();
                Ok(Self((v * match_suffix(&suffix) as f64) as u64))
            }
            Err(error) => Err(format!(
                "couldn't parse {:?} into a ByteSize, {}",
                value, error
            )),
        }
    }
}

/// todo: maybe a Unit type would be appropriate
fn match_suffix(unit: &str) -> u64 {
    match unit.to_lowercase().as_str() {
        "k" | "kb" => super::KB,
        "ki" | "kib" => super::KIB,
        "m" | "mb" => super::MB,
        "mi" | "mib" => super::MIB,
        "g" | "gb" => super::GB,
        "gi" | "gib" => super::GIB,
        "t" | "tb" => super::TB,
        "ti" | "tib" => super::TIB,
        "p" | "pb" => super::PB,
        "pi" | "pib" => super::PIB,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{KB, KIB, MB, MIB, GB, GIB, TB, TIB, PB, PIB};

    #[test]
    fn when_ok() {
        // shortcut for writing test cases
        fn parse(s: &str) -> u64 {
            s.parse::<ByteSize>().unwrap().0
        }

        assert_eq!("0".parse::<ByteSize>().unwrap().0, 0);
        assert_eq!(parse("0"), 0);
        assert_eq!(parse("500"), 500);
        assert_eq!(parse("1K"), 1 * KB);
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
        assert_eq!(parse("12 PiB"), 12 * PIB);
    }

    #[test]
    fn when_err() {
        // shortcut for writing test cases
        fn parse(s: &str) -> Result<ByteSize, String> {
            s.parse::<ByteSize>()
        }

        assert!(parse("").is_err());
        assert!(parse("a124GB").is_err());
    }
}

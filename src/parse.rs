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
                match suffix.parse::<Unit>() {
                    Ok(u) => Ok(Self((v * u.0 as f64) as u64)),
                    Err(error) => Err(format!(
                        "couldn't parse {:?} into a known SI unit, {}",
                        suffix, error
                    )),
                }
            }
            Err(error) => Err(format!(
                "couldn't parse {:?} into a ByteSize, {}",
                value, error
            )),
        }
    }
}

#[repr(transparent)]
struct Unit(pub(crate) u64);

impl Unit {
    const KB: u64 = super::KB;
    const KIB: u64 = super::KIB;
    const MB: u64 = super::MB;
    const MIB: u64 = super::MIB;
    const GB: u64 = super::GB;
    const GIB: u64 = super::GIB;
    const TB: u64 = super::TB;
    const TIB: u64 = super::TIB;
    const PB: u64 = super::PB;
    const PIB: u64 = super::PIB;
}

impl std::str::FromStr for Unit {
    type Err = String;

    fn from_str(unit: &str) -> Result<Self, Self::Err> {
        match unit.to_lowercase().as_str() {
            "k" | "kb" => Ok(Self(Self::KB)),
            "ki" | "kib" => Ok(Self(Self::KIB)),
            "m" | "mb" => Ok(Self(Self::MB)),
            "mi" | "mib" => Ok(Self(Self::MIB)),
            "g" | "gb" => Ok(Self(Self::GB)),
            "gi" | "gib" => Ok(Self(Self::GIB)),
            "t" | "tb" => Ok(Self(Self::TB)),
            "ti" | "tib" => Ok(Self(Self::TIB)),
            "p" | "pb" => Ok(Self(Self::PB)),
            "pi" | "pib" => Ok(Self(Self::PIB)),
            _ => Err(format!("couldn't parse unit of {:?}", unit)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_ok() {
        // shortcut for writing test cases
        fn parse(s: &str) -> u64 {
            s.parse::<ByteSize>().unwrap().0
        }

        assert_eq!("0".parse::<ByteSize>().unwrap().0, 0);
        assert_eq!(parse("0"), 0);
        assert_eq!(parse("500"), 500);
        assert_eq!(parse("1K"), 1 * Unit::KB);
        assert_eq!(parse("1Ki"), 1 * Unit::KIB);
        assert_eq!(parse("1.5Ki"), (1.5 * Unit::KIB as f64) as u64);
        assert_eq!(parse("1KiB"), 1 * Unit::KIB);
        assert_eq!(parse("1.5KiB"), (1.5 * Unit::KIB as f64) as u64);
        assert_eq!(parse("3 MB"), 3 * Unit::MB);
        assert_eq!(parse("4 MiB"), 4 * Unit::MIB);
        assert_eq!(parse("6 GB"), 6 * Unit::GB);
        assert_eq!(parse("4 GiB"), 4 * Unit::GIB);
        assert_eq!(parse("88TB"), 88 * Unit::TB);
        assert_eq!(parse("521TiB"), 521 * Unit::TIB);
        assert_eq!(parse("8 PB"), 8 * Unit::PB);
        assert_eq!(parse("12 PiB"), 12 * Unit::PIB);
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

    #[test]
    fn to_and_from_str() {
        assert_eq!(
            format!("{}", "128GB".parse::<ByteSize>().unwrap())
                .parse::<ByteSize>()
                .unwrap()
                .0,
            128 * Unit::GB
        );
        assert_eq!(
            super::super::to_string("128.000 GiB".parse::<ByteSize>().unwrap().0, true)
                .parse::<ByteSize>()
                .unwrap()
                .0,
            128 * Unit::GIB
        );
    }
}

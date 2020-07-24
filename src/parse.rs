use super::ByteSize;

impl std::str::FromStr for ByteSize {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = value.parse::<u64>() {
            return Ok(Self(v));
        }
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
                    Ok(u) => Ok(Self((v * u.factor() as f64) as u64)),
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

enum Unit {
    Byte,
    // power of tens
    KiloByte,
    MegaByte,
    GigaByte,
    TeraByte,
    PetaByte,
    // power of twos
    KibiByte,
    MebiByte,
    GibiByte,
    TebiByte,
    PebiByte,
}

impl Unit {
    fn factor(&self) -> u64 {
        match self {
            Self::Byte => super::B,
            // power of tens
            Self::KiloByte => super::KB,
            Self::MegaByte => super::MB,
            Self::GigaByte => super::GB,
            Self::TeraByte => super::TB,
            Self::PetaByte => super::PB,
            // power of twos
            Self::KibiByte => super::KIB,
            Self::MebiByte => super::MIB,
            Self::GibiByte => super::GIB,
            Self::TebiByte => super::TIB,
            Self::PebiByte => super::PIB,
        }
    }
}

impl std::str::FromStr for Unit {
    type Err = String;

    fn from_str(unit: &str) -> Result<Self, Self::Err> {
        match unit.to_lowercase().as_str() {
            "b" => Ok(Self::Byte),
            // power of tens
            "k" | "kb" => Ok(Self::KiloByte),
            "m" | "mb" => Ok(Self::MegaByte),
            "g" | "gb" => Ok(Self::GigaByte),
            "t" | "tb" => Ok(Self::TeraByte),
            "p" | "pb" => Ok(Self::PetaByte),
            // power of twos
            "ki" | "kib" => Ok(Self::KibiByte),
            "mi" | "mib" => Ok(Self::MebiByte),
            "gi" | "gib" => Ok(Self::GibiByte),
            "ti" | "tib" => Ok(Self::TebiByte),
            "pi" | "pib" => Ok(Self::PebiByte),
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
        assert_eq!(parse("1K"), 1 * crate::KB);
        assert_eq!(parse("1Ki"), 1 * crate::KIB);
        assert_eq!(parse("1.5Ki"), (1.5 * crate::KIB as f64) as u64);
        assert_eq!(parse("1KiB"), 1 * crate::KIB);
        assert_eq!(parse("1.5KiB"), (1.5 * crate::KIB as f64) as u64);
        assert_eq!(parse("3 MB"), 3 * crate::MB);
        assert_eq!(parse("4 MiB"), 4 * crate::MIB);
        assert_eq!(parse("6 GB"), 6 * crate::GB);
        assert_eq!(parse("4 GiB"), 4 * crate::GIB);
        assert_eq!(parse("88TB"), 88 * crate::TB);
        assert_eq!(parse("521TiB"), 521 * crate::TIB);
        assert_eq!(parse("8 PB"), 8 * crate::PB);
        assert_eq!(parse("8P"), 8 * crate::PB);
        assert_eq!(parse("12 PiB"), 12 * crate::PIB);
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
        // shortcut for writing test cases
        fn parse(s: &str) -> u64 {
            s.parse::<ByteSize>().unwrap().0
        }

        assert_eq!(parse(&format!("{}", parse("128GB"))), 128 * crate::GB);
        assert_eq!(
            parse(&crate::to_string(parse("128.000 GiB"), true)),
            128 * crate::GIB
        );
    }
}

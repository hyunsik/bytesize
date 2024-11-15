use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ByteLike)]
pub fn bytelike(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let constructor = bytelike_constructor(input_str.parse().unwrap());
    let display = bytelike_display(input_str.parse().unwrap());
    let parse = bytelike_parse(input_str.parse().unwrap());
    let arithmetic = bytelike_arithmetic(input_str.parse().unwrap());
    let fromstr = bytelike_fromstr(input_str.parse().unwrap());
    
    let mut combined = format!(
        "{}{}{}{}{}",
        constructor, display, parse, arithmetic, fromstr
    );
    if cfg!(feature = "serde") {
        let serde = bytelike_serde(input_str.parse().unwrap());
        combined = format!("{}{}", combined, serde);
    }
    combined.parse().unwrap()
}

#[proc_macro_derive(ByteLikeConstructor)]
pub fn bytelike_constructor(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl #name {
            #[inline(always)]
            pub const fn b(size: u64) -> Self {
                Self(size)
            }

            #[inline(always)]
            pub const fn kb(size: u64) -> Self {
                Self(size * bytelike::KB)
            }

            #[inline(always)]
            pub const fn kib(size: u64) -> Self {
                Self(size * bytelike::KIB)
            }

            #[inline(always)]
            pub const fn mb(size: u64) -> Self {
                Self(size * bytelike::MB)
            }

            #[inline(always)]
            pub const fn mib(size: u64) -> Self {
                Self(size * bytelike::MIB)
            }

            #[inline(always)]
            pub const fn gb(size: u64) -> Self {
                Self(size * bytelike::GB)
            }

            #[inline(always)]
            pub const fn gib(size: u64) -> Self {
                Self(size * bytelike::GIB)
            }

            #[inline(always)]
            pub const fn tb(size: u64) -> Self {
                Self(size * bytelike::TB)
            }

            #[inline(always)]
            pub const fn tib(size: u64) -> Self {
                Self(size * bytelike::TIB)
            }

            #[inline(always)]
            pub const fn pb(size: u64) -> Self {
                Self(size * bytelike::PB)
            }

            #[inline(always)]
            pub const fn pib(size: u64) -> Self {
                Self(size * bytelike::PIB)
            }
        }

        impl From<u64> for #name {
            fn from(size: u64) -> #name {
                Self(size)
            }
        }

        impl From<#name> for u64 {
            fn from(s: #name) -> u64 {
                s.0
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ByteLikeArithmetic)]
pub fn bytelike_arithmetic(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl std::ops::Add<#name> for #name {
            type Output = #name;

            #[inline(always)]
            fn add(self, rhs: #name) -> #name {
                #name(self.0 + rhs.0)
            }
        }

        impl std::ops::AddAssign<#name> for #name {
            #[inline(always)]
            fn add_assign(&mut self, rhs: #name) {
                self.0 += rhs.0
            }
        }

        impl<T> std::ops::MulAssign<T> for #name
        where
            T: Into<u64>,
        {
            #[inline(always)]
            fn mul_assign(&mut self, rhs: T) {
                self.0 *= rhs.into();
            }
        }

        impl std::ops::Add<#name> for u32 {
            type Output = #name;
            #[inline(always)]
            fn add(self, rhs: #name) -> #name {
                #name(rhs.0 + (self as u64))
            }
        }

        impl std::ops::Add<#name> for u16 {
            type Output = #name;
            #[inline(always)]
            fn add(self, rhs: #name) -> #name {
                #name(rhs.0 + (self as u64))
            }
        }

        impl std::ops::Add<#name> for u8 {
            type Output = #name;
            #[inline(always)]
            fn add(self, rhs: #name) -> #name {
                #name(rhs.0 + (self as u64))
            }
        }

        impl std::ops::Mul<#name> for u64 {
            type Output = #name;
            #[inline(always)]
            fn mul(self, rhs: #name) -> #name {
                #name(rhs.0 * self)
            }
        }

        impl std::ops::Mul<#name> for u32 {
            type Output = #name;
            #[inline(always)]
            fn mul(self, rhs: #name) -> #name {
                #name(rhs.0 * (self as u64))
            }
        }

        impl std::ops::Mul<#name> for u16 {
            type Output = #name;
            #[inline(always)]
            fn mul(self, rhs: #name) -> #name {
                #name(rhs.0 * (self as u64))
            }
        }

        impl std::ops::Mul<#name> for u8 {
            type Output = #name;
            #[inline(always)]
            fn mul(self, rhs: #name) -> #name {
                #name(rhs.0 * (self as u64))
            }
        }

        impl #name {
            fn range<I: Into<Self>>(start: I, stop: I) -> bytelike::ByteLikeRange<Self> {
                bytelike::ByteLikeRange::new(Some(start.into()), Some(stop.into()))
            }
        
            fn range_from<I: Into<Self>>(start: I) -> bytelike::ByteLikeRange<Self> {
                bytelike::ByteLikeRange::new(Some(start.into()), None)
            }
        
            fn range_to<I: Into<Self>>(stop: I) -> bytelike::ByteLikeRange<Self> {
                bytelike::ByteLikeRange::new(None, Some(stop.into()))
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ByteLikeDisplay)]
pub fn bytelike_display(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.pad(&bytelike::to_string(self.0, true))
            }
        }

        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ByteLikeFromStr)]
pub fn bytelike_fromstr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl std::str::FromStr for #name {
            type Err = String;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                if let Ok(v) = value.parse::<u64>() {
                    return Ok(Self(v));
                }
                let number = bytelike::take_while(value, |c| c.is_ascii_digit() || c == '.');
                match number.parse::<f64>() {
                    Ok(v) => {
                        let suffix = bytelike::skip_while(value, |c| {
                            c.is_whitespace() || c.is_ascii_digit() || c == '.'
                        });
                        match suffix.parse::<bytelike::Unit>() {
                            Ok(u) => Ok(Self((v * u64::from(u) as f64) as u64)),
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
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ByteLikeParse)]
pub fn bytelike_parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl #name {
            #[inline(always)]
            pub fn to_string_as(&self, si_unit: bool) -> String {
                bytelike::to_string(self.0, si_unit)
            }

            #[inline(always)]
            pub const fn as_u64(&self) -> u64 {
                self.0
            }
        }
    };

    TokenStream::from(expanded)
}
#[proc_macro_derive(ByteLikeSerde)]
pub fn bytelike_serde(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct ByteSizeVistor;

                impl<'de> serde::de::Visitor<'de> for ByteSizeVistor {
                    type Value = #name;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        formatter.write_str("an integer or string")
                    }

                    fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
                        if let Ok(val) = u64::try_from(value) {
                            Ok(#name(val))
                        } else {
                            Err(E::invalid_value(
                                serde::de::Unexpected::Signed(value),
                                &"integer overflow",
                            ))
                        }
                    }

                    fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
                        Ok(#name(value))
                    }

                    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                        if let Ok(val) = value.parse() {
                            Ok(val)
                        } else {
                            Err(E::invalid_value(
                                serde::de::Unexpected::Str(value),
                                &"parsable string",
                            ))
                        }
                    }
                }

                if deserializer.is_human_readable() {
                    deserializer.deserialize_any(ByteSizeVistor)
                } else {
                    deserializer.deserialize_u64(ByteSizeVistor)
                }
            }
        }
        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                if serializer.is_human_readable() {
                    <str>::serialize(self.to_string().as_str(), serializer)
                } else {
                    self.0.serialize(serializer)
                }
            }
        }
    };

    TokenStream::from(expanded)
}

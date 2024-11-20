use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use proc_macro2::Span;

#[proc_macro_derive(ByteLike)]
pub fn bytelike(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let constructor = bytelike_constructor(input_str.parse().unwrap());
    let display = bytelike_display(input_str.parse().unwrap());
    let parse = bytelike_parse(input_str.parse().unwrap());
    let ops = bytelike_ops(input_str.parse().unwrap());
    let fromstr = bytelike_fromstr(input_str.parse().unwrap());

    let mut combined = format!("{}{}{}{}{}", constructor, display, parse, ops, fromstr);
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

    // Define units with their multipliers and descriptions
    let units = vec![
        ("b", "1", "bytes"),
        ("kb", "bytelike::KB", "kilobytes"),
        ("kib", "bytelike::KIB", "kibibytes"),
        ("mb", "bytelike::MB", "megabytes"),
        ("mib", "bytelike::MIB", "mebibytes"),
        ("gb", "bytelike::GB", "gigabytes"),
        ("gib", "bytelike::GIB", "gibibytes"),
        ("tb", "bytelike::TB", "terabytes"),
        ("tib", "bytelike::TIB", "tebibytes"),
        ("pb", "bytelike::PB", "petabytes"),
        ("pib", "bytelike::PIB", "pebibytes"),
    ];

    // Generate methods
    let methods = units.iter().map(|(fn_name, multiplier, description)| {
        // Create an identifier for the method name
        let method_name = syn::Ident::new(fn_name, Span::call_site());

        // Parse the multiplier into an expression
        let multiplier_expr: syn::Expr = syn::parse_str(multiplier).unwrap();

        // Generate the documentation comment
        let doc_comment = format!("Construct `{}` given an amount of {}.", name, description);

        // Generate the method using quote!
        quote! {
            #[doc = #doc_comment]
            #[inline(always)]
            pub const fn #method_name(size: u64) -> Self {
                Self(size * #multiplier_expr)
            }
        }
    });

    let expanded = quote! {
        impl #name {
            #(#methods)*
        }

        impl From<u64> for #name {
            fn from(size: u64) -> #name {
                Self(size)
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(ByteLikeOps)]
pub fn bytelike_ops(input: TokenStream) -> TokenStream {
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

        impl<T> std::ops::Add<T> for #name
        where
            T: Into<u64>,
        {
            type Output = #name;
            #[inline(always)]
            fn add(self, rhs: T) -> #name {
                #name(self.0 + (rhs.into()))
            }
        }

        impl<T> std::ops::AddAssign<T> for #name
        where
            T: Into<u64>,
        {
            #[inline(always)]
            fn add_assign(&mut self, rhs: T) {
                self.0 += rhs.into();
            }
        }
        
        impl std::ops::Sub<#name> for #name {
            type Output = #name;
        
            #[inline(always)]
            fn sub(self, rhs: #name) -> #name {
                #name(self.0 - rhs.0)
            }
        }
        
        impl std::ops::SubAssign<#name> for #name {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: #name) {
                self.0 -= rhs.0
            }
        }
        
        impl<T> std::ops::Sub<T> for #name
        where
            T: Into<u64>,
        {
            type Output = #name;
        
            #[inline(always)]
            fn sub(self, rhs: T) -> #name {
                #name(self.0 - (rhs.into()))
            }
        }
        
        impl<T> std::ops::SubAssign<T> for #name
        where
            T: Into<u64>,
        {
            #[inline(always)]
            fn sub_assign(&mut self, rhs: T) {
                self.0 -= rhs.into();
            }
        }
        
        impl<T> std::ops::Mul<T> for #name
        where
            T: Into<u64>,
        {
            type Output = #name;
            #[inline(always)]
            fn mul(self, rhs: T) -> #name {
                #name(self.0 * rhs.into())
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

        impl std::ops::Add<#name> for u64 {
            type Output = #name;
            #[inline(always)]
            fn add(self, rhs: #name) -> #name {
                #name(rhs.0 + self)
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
            /// Provides `ByteLikeRange` with explicit lower and upper bounds.
            pub fn range<I: Into<Self>>(start: I, stop: I) -> bytelike::ByteLikeRange<Self> {
                bytelike::ByteLikeRange::new(Some(start), Some(stop))
            }

            /// Provides `ByteLikeRange` with explicit lower bound. Upper bound is set to `u64::MAX`.
            pub fn range_start<I: Into<Self>>(start: I) -> bytelike::ByteLikeRange<Self> {
                bytelike::ByteLikeRange::new(Some(start), None)
            }
            
            /// Provides `ByteLikeRange` with explicit lower bound. Upper bound is set to `u64::MAX`.
            pub fn range_stop<I: Into<Self>>(stop: I) -> bytelike::ByteLikeRange<Self> {
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
            /// Returns the size as a string with an optional SI unit.
            #[inline(always)]
            pub fn to_string_as(&self, si_unit: bool) -> String {
                bytelike::to_string(self.0, si_unit)
            }

            /// Returns the inner u64 value.
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

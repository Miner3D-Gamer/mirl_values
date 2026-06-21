#![allow(trivial_numeric_casts)]
#![allow(clippy::cast_lossless)]
use std::convert::TryFrom;

use num_bigfloat::BigFloat;
use num_bigint::BigInt;

// #[derive(Debug, Clone, PartialEq, PartialOrd)]
// pub enum Number {
//     Int(i128),
//     Float(f64),
//     BigInt(num_bigint::BigInt),
//     BigFloat(num_bigfloat::BigFloat),
// }
// ── From<T> for Number ────────────────────────────────────────────────────────

// Signed integers → Int
macro_rules! impl_from_signed_int {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Number {
                #[inline]
                fn from(v: $t) -> Self {
                    Number::Int(v as i128)
                }
            }
        )*
    };
}

impl_from_signed_int!(i8, i16, i32, i64, i128, isize);

// Unsigned integers → Int (all fit in i128 except u128 which needs BigInt for large values)
macro_rules! impl_from_unsigned_int {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Number {
                #[inline]
                fn from(v: $t) -> Self {
                    Number::Int(v as i128)
                }
            }
        )*
    };
}

impl_from_unsigned_int!(u8, u16, u32, u64, usize);

// u128 → Int if it fits, otherwise BigInt
impl From<u128> for Number {
    #[inline]
    fn from(v: u128) -> Self {
        if v <= i128::MAX as u128 {
            Self::Int(v as i128)
        } else {
            Self::BigInt(BigInt::from(v))
        }
    }
}

// f32, f64 → Float
impl From<f32> for Number {
    #[inline]
    fn from(v: f32) -> Self {
        Self::Float(v as f64)
    }
}

impl From<f64> for Number {
    #[inline]
    fn from(v: f64) -> Self {
        Self::Float(v)
    }
}

// f16 → Float
impl From<f16> for Number {
    #[inline]
    fn from(v: f16) -> Self {
        Self::Float(v as f64)
    }
}

// f128 → BigFloat  (requires the `f128` crate or nightly; here we use num-bigfloat directly)
// On nightly you can enable #![feature(f128)] and use the primitive f128 type.
// On stable, wrap your f128 value in a new type or use BigFloat directly.
//
// Nightly primitive f128 implementation (uncomment when using nightly + feature(f128)):
// #[cfg(feature = "nightly_f128")]
// impl From<f128> for Number {
//     #[inline]
//     fn from(v: f128) -> Self {
//         // BigFloat::parse accepts a decimal string representation
//         Number::BigFloat(BigFloat::parse(&format!("{:.}", v)).unwrap_or(BigFloat::nan()))
//     }
// }

// BigInt → Number
impl From<BigInt> for Number {
    #[inline]
    fn from(v: BigInt) -> Self {
        Self::BigInt(v)
    }
}

// BigFloat → Number
impl From<BigFloat> for Number {
    #[inline]
    fn from(v: BigFloat) -> Self {
        Self::BigFloat(v)
    }
}

// ── TryFrom<Number> for primitive types ──────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberConversionError(pub String);

impl std::fmt::Display for NumberConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Number conversion error: {}", self.0)
    }
}

impl std::error::Error for NumberConversionError {}

/// Convert the value to i128
// TODO: convert a Number to i128 as an intermediate step
pub fn number_to_i128(n: &Number) -> Result<i128, NumberConversionError> {
    match n {
        Number::Int(i) => Ok(*i),
        Number::Float(f) => {
            if f.fract() != 0.0 {
                return Err(NumberConversionError(format!(
                    "float {f} has a fractional part"
                )));
            }
            if *f < i128::MIN as f64 || *f > i128::MAX as f64 {
                return Err(NumberConversionError(format!(
                    "float {f} out of i128 range"
                )));
            }
            Ok(*f as i128)
        }
        Number::BigInt(b) => {
            use num_traits::ToPrimitive;
            b.to_i128().ok_or_else(|| {
                NumberConversionError(format!("BigInt {b} out of i128 range"))
            })
        }
        Number::BigFloat(b) => {
            // Convert BigFloat → f64 → i128 as best-effort
            let f = b.to_f64();
            if f.fract() != 0.0 {
                return Err(NumberConversionError(
                    "BigFloat has a fractional part".into(),
                ));
            }
            if f < i128::MIN as f64 || f > i128::MAX as f64 {
                return Err(NumberConversionError(
                    "BigFloat out of i128 range".into(),
                ));
            }
            Ok(f as i128)
        }
    }
}

// Helper: convert a Number to f64
fn number_to_f64(n: &Number) -> Result<f64, NumberConversionError> {
    match n {
        Number::Int(i) => Ok(*i as f64),
        Number::Float(f) => Ok(*f),
        Number::BigInt(b) => {
            use num_traits::ToPrimitive;
            b.to_f64().ok_or_else(|| {
                NumberConversionError(
                    "BigInt cannot be represented as f64".into(),
                )
            })
        }
        Number::BigFloat(b) => Ok(b.to_f64()),
    }
}

// Macro for signed integer TryFrom
macro_rules! impl_try_from_number_signed {
    ($($t:ty),*) => {
        $(
            impl TryFrom<Number> for $t {
                type Error = NumberConversionError;

                fn try_from(n: Number) -> Result<Self, Self::Error> {
                    let i = number_to_i128(&n)?;
                    <$t>::try_from(i).map_err(|_| {
                        NumberConversionError(format!(
                            "value {i} out of range for {}",
                            stringify!($t)
                        ))
                    })
                }
            }

            impl TryFrom<&Number> for $t {
                type Error = NumberConversionError;

                fn try_from(n: &Number) -> Result<Self, Self::Error> {
                    let i = number_to_i128(n)?;
                    <$t>::try_from(i).map_err(|_| {
                        NumberConversionError(format!(
                            "value {i} out of range for {}",
                            stringify!($t)
                        ))
                    })
                }
            }
        )*
    };
}

impl_try_from_number_signed!(i8, i16, i32, i64, i128, isize);

// Macro for unsigned integer TryFrom
macro_rules! impl_try_from_number_unsigned {
    ($($t:ty),*) => {
        $(
            impl TryFrom<Number> for $t {
                type Error = NumberConversionError;

                fn try_from(n: Number) -> Result<Self, Self::Error> {
                    let i = number_to_i128(&n)?;
                    <$t>::try_from(i).map_err(|_| {
                        NumberConversionError(format!(
                            "value {i} out of range for {}",
                            stringify!($t)
                        ))
                    })
                }
            }

            impl TryFrom<&Number> for $t {
                type Error = NumberConversionError;

                fn try_from(n: &Number) -> Result<Self, Self::Error> {
                    let i = number_to_i128(n)?;
                    <$t>::try_from(i).map_err(|_| {
                        NumberConversionError(format!(
                            "value {i} out of range for {}",
                            stringify!($t)
                        ))
                    })
                }
            }
        )*
    };
}

impl_try_from_number_unsigned!(u8, u16, u32, u64, u128, usize);

// f32 TryFrom
impl TryFrom<Number> for f32 {
    type Error = NumberConversionError;

    fn try_from(n: Number) -> Result<Self, Self::Error> {
        let f = number_to_f64(&n)?;
        if f < Self::MIN as f64 || f > Self::MAX as f64 {
            return Err(NumberConversionError(format!(
                "value {f} out of range for f32"
            )));
        }
        Ok(f as Self)
    }
}

impl TryFrom<&Number> for f32 {
    type Error = NumberConversionError;

    fn try_from(n: &Number) -> Result<Self, Self::Error> {
        let f = number_to_f64(n)?;
        if f < Self::MIN as f64 || f > Self::MAX as f64 {
            return Err(NumberConversionError(format!(
                "value {f} out of range for f32"
            )));
        }
        Ok(f as Self)
    }
}

// f64 TryFrom
impl TryFrom<Number> for f64 {
    type Error = NumberConversionError;

    fn try_from(n: Number) -> Result<Self, Self::Error> {
        number_to_f64(&n)
    }
}

impl TryFrom<&Number> for f64 {
    type Error = NumberConversionError;

    fn try_from(n: &Number) -> Result<Self, Self::Error> {
        number_to_f64(n)
    }
}

impl TryFrom<Number> for f16 {
    type Error = NumberConversionError;

    fn try_from(n: Number) -> Result<Self, Self::Error> {
        let f = number_to_f64(&n)?;
        let f16_val = f as Self;
        if f16_val.is_infinite() && !f.is_infinite() {
            return Err(NumberConversionError(format!(
                "value {f} out of range for f16"
            )));
        }
        Ok(f16_val)
    }
}

impl TryFrom<&Number> for f16 {
    type Error = NumberConversionError;

    fn try_from(n: &Number) -> Result<Self, Self::Error> {
        let f = number_to_f64(n)?;
        let f16_val = f as Self;
        if f16_val.is_infinite() && !f.is_infinite() {
            return Err(NumberConversionError(format!(
                "value {f} out of range for f16"
            )));
        }
        Ok(f16_val)
    }
}

// f128 TryFrom (nightly only — uncomment when using #![feature(f128)])
//
// #[cfg(feature = "nightly_f128")]
// impl TryFrom<Number> for f128 {
//     type Error = NumberConversionError;
//
//     fn try_from(n: Number) -> Result<Self, Self::Error> {
//         match n {
//             Number::Int(i) => Ok(i as f128),
//             Number::Float(f) => Ok(f as f128),
//             Number::BigInt(ref b) => {
//                 use num_traits::ToPrimitive;
//                 b.to_f64()
//                     .map(|f| f as f128)
//                     .ok_or_else(|| NumberConversionError("BigInt too large for f128".into()))
//             }
//             Number::BigFloat(ref b) => Ok(b.to_f64() as f128),
//         }
//     }
// }

// BigInt TryFrom
impl TryFrom<Number> for BigInt {
    type Error = NumberConversionError;

    fn try_from(n: Number) -> Result<Self, Self::Error> {
        match n {
            Number::Int(i) => Ok(Self::from(i)),
            Number::Float(f) => {
                if f.fract() != 0.0 {
                    return Err(NumberConversionError(format!(
                        "float {f} has a fractional part"
                    )));
                }
                Ok(Self::from(f as i128))
            }
            Number::BigInt(b) => Ok(b),
            Number::BigFloat(b) => {
                let f = b.to_f64();
                if f.fract() != 0.0 {
                    return Err(NumberConversionError(
                        "BigFloat has a fractional part".into(),
                    ));
                }
                Ok(Self::from(f as i128))
            }
        }
    }
}

impl TryFrom<&Number> for BigInt {
    type Error = NumberConversionError;

    fn try_from(n: &Number) -> Result<Self, Self::Error> {
        match n {
            Number::Int(i) => Ok(Self::from(*i)),
            Number::Float(f) => {
                if f.fract() != 0.0 {
                    return Err(NumberConversionError(format!(
                        "float {f} has a fractional part"
                    )));
                }
                Ok(Self::from(*f as i128))
            }
            Number::BigInt(b) => Ok(b.clone()),
            Number::BigFloat(b) => {
                let f = b.to_f64();
                if f.fract() != 0.0 {
                    return Err(NumberConversionError(
                        "BigFloat has a fractional part".into(),
                    ));
                }
                Ok(Self::from(f as i128))
            }
        }
    }
}

// BigFloat TryFrom (infallible, but keeping TryFrom for API consistency)
impl TryFrom<Number> for BigFloat {
    type Error = NumberConversionError;

    fn try_from(n: Number) -> Result<Self, Self::Error> {
        match n {
            Number::Int(i) => Ok(Self::from_i128(i)),
            Number::Float(f) => Ok(Self::from_f64(f)),
            Number::BigInt(b) => {
                use num_traits::ToPrimitive;
                let f = b.to_f64().ok_or_else(|| {
                    NumberConversionError(
                        "BigInt too large for BigFloat".into(),
                    )
                })?;
                Ok(Self::from_f64(f))
            }
            Number::BigFloat(b) => Ok(b),
        }
    }
}

impl TryFrom<&Number> for BigFloat {
    type Error = NumberConversionError;

    fn try_from(n: &Number) -> Result<Self, Self::Error> {
        match n {
            Number::Int(i) => Ok(Self::from_i128(*i)),
            Number::Float(f) => Ok(Self::from_f64(*f)),
            Number::BigInt(b) => {
                use num_traits::ToPrimitive;
                let f = b.to_f64().ok_or_else(|| {
                    NumberConversionError(
                        "BigInt too large for BigFloat".into(),
                    )
                })?;
                Ok(Self::from_f64(f))
            }
            Number::BigFloat(b) => Ok(*b),
        }
    }
}

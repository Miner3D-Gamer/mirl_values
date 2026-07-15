//! A lib that more or less supports all formats
//!
//! Here is the Problem:
//!     - `serde_json` doesn't support datetime from `toml`
//!     - `toml` doesn't support value length from `css`
//!     - and so on, everyone has their own "Value" enum(/struct)
//!
//! So what if we unify it every single unique "Value" enum(/struct) into a super [`Value`](crate::values::Value)?
// TODO: Add simd_json support

/// The settings for the crate
pub mod settings {
    pub use mirl_values_core::settings::*;
}
/// What values are expected to always be imported
pub mod prelude;

/// The actual values
pub mod values;

use mirl_extensions::*;
use mirl_values_core::value::ContainerValue;
pub use values::Value;

// pub use values::{SimpleValue, ValueType};

#[must_use]
/// Convert a map to a serde json map
#[cfg(feature = "serde_json")]
pub fn to_serde_json_map(
    val: crate::settings::MapType<Value<DefaultInnerValueSelf>, Value<DefaultInnerValueSelf>>,
) -> Option<serde_json::Value> {
    let mut map = serde_json::map::Map::new();
    for (key, item) in val {
        map.insert(key.as_string_ref()?.clone(), item.to_serde_json()?);
    }

    Some(serde_json::Value::Object(map))
}

use crate::values::value::{DefaultInnerValueSelf, InnerCodecValue};

impl<W: InnerCodecValue> Value<W> {
    // #[must_use]
    // /// Get the value type of the value
    // pub const fn get_value_type(&self) -> ValueType {
    //     match self {
    //         Self::Simple(simple) => match simple {
    //             SimpleValue::None => ValueType::None,
    //             SimpleValue::Bool(_) => ValueType::Bool,
    //             SimpleValue::Number(_) => ValueType::Number,
    //             SimpleValue::String(_) => ValueType::String,
    //             SimpleValue::Time(_) => ValueType::Time,
    //             SimpleValue::DateTime(_) => ValueType::DateTime,
    //             SimpleValue::Angle(_, _) => ValueType::Angle,
    //             SimpleValue::Literal(_) => ValueType::Literal,
    //             SimpleValue::Length(_, _) => ValueType::Length,
    //             SimpleValue::Color(_) => ValueType::Color,
    //             SimpleValue::Bytes(_, _) => todo!(),
    //         },
    //         Self::Container(container) => match container {
    //             ContainerValue::Vec(_) => ValueType::Vec,
    //             ContainerValue::Map(_) => ValueType::Map,
    //         },
    //     }
    // }
    /// Get a sub sequent value, return None if it doesn't exist
    pub fn get<T: IndexValue<W> + ?Sized>(&self, val: &T) -> Option<&W::Inner> {
        val.index_value(self)
    }
}

/// A trait to index into a [Value]
pub trait IndexValue<W: InnerCodecValue> {
    /// Index into the given [Value] and return a [Value] if the requested [Value] exists, otherwise return [None]
    fn index_value<'a>(&self, v: &'a Value<W>) -> Option<&'a W::Inner>;

    /// Index into the given [Value] and return a mutable [Value] if the requested [Value] exists, otherwise return [None]
    fn index_value_mut<'a>(&self, v: &'a mut Value<W>) -> Option<&'a mut W::Inner>;
}

impl<W: InnerCodecValue> IndexValue<W> for usize {
    fn index_value<'a>(&self, v: &'a Value<W>) -> Option<&'a W::Inner> {
        #[allow(unreachable_patterns)]
        match v {
            Value::Container(container) => match container {
                ContainerValue::Vec(val) => val.get(*self),
                // #[cfg(feature = "preserve_entries")]
                ContainerValue::Map(val) => val.get_index(*self),
                _ => None,
            },
            Value::Simple(_) => None,
        }
    }
    fn index_value_mut<'a>(&self, v: &'a mut Value<W>) -> Option<&'a mut W::Inner> {
        #[allow(unreachable_patterns)]
        match v {
            Value::Container(container) => match container {
                ContainerValue::Vec(val) => val.get_mut(*self),
                // #[cfg(feature = "preserve_entries")]
                ContainerValue::Map(val) => val.get_index_mut(*self),
                _ => None,
            },
            Value::Simple(_) => None,
        }
    }
}

impl<W: InnerCodecValue> IndexValue<W> for str
where
    W::Inner: CodecSimpleSubValueRef + CodecSimpleSubValueRefMut,
{
    fn index_value<'a>(&self, v: &'a values::value::Value<W>) -> std::option::Option<&'a W::Inner> {
        match v {
            Value::Container(container) => match container {
                ContainerValue::Map(val) => {
                    let origin = self.to_string();
                    for (k, v) in val.iter() {
                        if let Some(s) = k.as_simple()?.as_string_ref()
                            && origin.eq(s)
                        {
                            return Some(v);
                        }
                    }
                    None
                }
                ContainerValue::Vec(_) => None,
            },
            Value::Simple(_) => None,
        }
    }
    fn index_value_mut<'a>(&self, v: &'a mut Value<W>) -> Option<&'a mut W::Inner> {
        match v {
            Value::Container(container) => match container {
                ContainerValue::Map(val) => {
                    let origin = self.to_string();
                    for (k, v) in val.iter_mut() {
                        if let Some(s) = k.as_simple()?.as_string_ref()
                            && origin.eq(s)
                        {
                            return Some(v);
                        }
                    }
                    None
                }
                ContainerValue::Vec(_) => None,
            },
            Value::Simple(_) => None,
        }
    }
}
#[must_use]
/// Convert a number to a serde value
#[cfg(feature = "serde_json")]
pub fn number_to_serde_json_number(
    num: mirl_values_core::value::Number,
) -> Option<serde_json::Number> {
    #[cfg(feature = "serde_json_arbitrary_precision")]
    return Some(match num {
        mirl_values_core::value::Number::Int(num) => {
            serde_json::Number::from_string_unchecked(num.to_string())
        }
        mirl_values_core::value::Number::Float(num) => {
            serde_json::Number::from_string_unchecked(num.to_string())
        }
        mirl_values_core::value::Number::BigInt(num) => {
            serde_json::Number::from_string_unchecked(num.to_string())
        }
        mirl_values_core::value::Number::BigFloat(num) => {
            serde_json::Number::from_string_unchecked(num.to_string())
        }
    });
    #[cfg(not(feature = "serde_json_arbitrary_precision"))]
    match num {
        mirl_values_core::value::Number::Int(num) => serde_json::Number::from_i128(num),
        mirl_values_core::value::Number::Float(num) => serde_json::Number::from_f64(num),
        mirl_values_core::value::Number::BigInt(num) => {
            use num_traits::ToPrimitive;

            serde_json::Number::from_i128(num.to_i128()?)
        }
        mirl_values_core::value::Number::BigFloat(num) => {
            serde_json::Number::from_f64(num.to_f64())
        }
    }
}
impl FromPatch<Self> for Value<DefaultInnerValueSelf> {
    fn from_value(value: Self) -> Self {
        value
    }
}

#[cfg(feature = "serde_json")]
impl<V: InnerCodecValue> Value<V>
where
    Self: IntoPatch<Value<DefaultInnerValueSelf>>,
{
    #[must_use]
    /// Convert the given [`Value`] into a [`serde_json::Value`] if possible.
    ///
    /// The given [`Value`] must implement [`IntoPatch`] to convert into [`Value<DefaultInnerValueSelf>`]
    pub fn to_serde_json(self) -> Option<serde_json::Value> {
        value_to_serde_json(self.into_value())
    }
}
#[cfg(feature = "serde_json")]
impl<V: InnerCodecValue> Value<V>
where
    Self: Into<Value<DefaultInnerValueSelf>>,
{
    #[must_use]
    /// Convert the given [`Value`] into a [`serde_json::Value`] if possible.
    ///
    /// The given [`Value`] must implement [`Into`] to convert into [`Value<DefaultInnerValueSelf>`]
    pub fn to_serde_json_fallback(self) -> Option<serde_json::Value> {
        value_to_serde_json(self.into())
    }
}
#[cfg(feature = "serde_json")]
use mirl_values_core::value::SimpleValue;
/// Convert a [`SimpleValue`] into a [`serde_json::Value`] if possible
#[cfg(feature = "serde_json")]
#[must_use]
pub fn simple_value_to_serde_json(simple: SimpleValue) -> Option<serde_json::Value> {
    match simple {
        SimpleValue::Bool(bool) => Some(serde_json::Value::Bool(bool)),
        SimpleValue::None => Some(serde_json::Value::Null),
        SimpleValue::Number(num) => {
            Some(serde_json::Value::Number(number_to_serde_json_number(num)?))
        }
        SimpleValue::String(string) => Some(serde_json::Value::String(string)),
        SimpleValue::Angle(_, _)
        | SimpleValue::Bytes(_, _)
        | SimpleValue::Color(_)
        | SimpleValue::DateTime(_)
        | SimpleValue::Length(_, _)
        | SimpleValue::Literal(_)
        | SimpleValue::Time(_) => None,
    }
}
/// Convert a [`ContainerValue`] into a [`serde_json::Value`] if possible
#[must_use]
#[cfg(feature = "serde_json")]
pub fn container_value_to_serde_json(
    container: ContainerValue<Value<DefaultInnerValueSelf>>,
) -> Option<serde_json::Value> {
    match container {
        ContainerValue::Map(map) => to_serde_json_map(crate::settings::MapType {
            map: map.into_iter().map(|x| (x.0, x.1)).collect::<Vec<_>>(),
        }),
        ContainerValue::Vec(vec) => {
            let list: Vec<Option<serde_json::Value>> = vec
                .into_iter()
                .map(values::value::Value::to_serde_json)
                .collect();
            if list.contains(&None) {
                return None;
            }
            let list = list
                .iter()
                .map(|x| unsafe { x.clone().unwrap_unchecked() })
                .collect();
            Some(serde_json::Value::Array(list))
        }
    }
}

#[cfg(feature = "serde_json")]
#[must_use]
/// Convert this value type to [`serde_json::Value`] uses, this looses the position information
pub fn value_to_serde_json(val: Value<DefaultInnerValueSelf>) -> Option<serde_json::Value> {
    match val {
        Value::Simple(simple) => simple_value_to_serde_json(simple),
        Value::Container(container) => container_value_to_serde_json(container),
    }
}

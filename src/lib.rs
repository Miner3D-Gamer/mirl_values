//! A lib that more or less supports all formats
//!
//! Problem:
//!     - `serde_json` doesn't support datetime from `toml`
//!     - `toml` doesn't support value length from `css`
//!     - and so on...
//!
//! So what if we unify it all into a super [`Value`](crate::values::Value)?
#![feature(f16)]
// #![feature(f128)]

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

// pub use values::{SimpleValue, ValueType};
// #[cfg(feature = "serde")]
// fn to_serde_json_map(val: &MapType<Value, Value>) -> Option<serde_json::Value> {
//     let mut map = serde_json::map::Map::new();
//     for (key, item) in val {
//         map.insert(key.as_string()?.clone(), item.to_serde_json()?);
//     }

//     Some(serde_json::Value::Object(map))
// }
use crate::values::value::{InnerCodecValue, Value};

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
    fn index_value_mut<'a>(
        &self,
        v: &'a mut Value<W>,
    ) -> Option<&'a mut W::Inner>;
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
    fn index_value_mut<'a>(
        &self,
        v: &'a mut Value<W>,
    ) -> Option<&'a mut W::Inner> {
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
    fn index_value<'a>(
        &self,
        v: &'a values::value::Value<W>,
    ) -> std::option::Option<&'a W::Inner> {
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
    fn index_value_mut<'a>(
        &self,
        v: &'a mut Value<W>,
    ) -> Option<&'a mut W::Inner> {
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

#[cfg(feature = "serde_json")]
impl Value<PositionedValueInner> {
    #[must_use]
    /// Convert this value type to [`serde_json::Value`] uses, this looses the position information
    pub fn to_serde_json(&self) -> Option<serde_json::Value> {
        use std::str::FromStr;

        Some(match self {
            Self::Vec(val, _) => {
                let list: Vec<Option<serde_json::Value>> =
                    val.iter().map(Self::to_serde_json).collect();
                if list.contains(&None) {
                    return None;
                }
                let list = list
                    .iter()
                    .map(|x| unsafe { x.clone().unwrap_unchecked() })
                    .collect();
                serde_json::Value::Vec(list)
            }
            Self::Bool(val, _) => serde_json::Value::Bool(*val),
            Self::Map(map, _) => to_serde_json_map(map)?,
            Self::Number(num, _) => serde_json::Value::Number(
                serde_json::Number::from_str(num).ok()?,
            ),
            Self::String(val, _, _) => serde_json::Value::String(val.clone()),
            Self::None(_) => serde_json::Value::Null,
        })
    }
}

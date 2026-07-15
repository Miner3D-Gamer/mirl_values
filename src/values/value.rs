// use crate::settings::*;
pub use mirl_extensions::InnerCodecValue;
use mirl_extensions::*;

use super::*;

/// The default impl for [`Value`]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default, PartialOrd, Ord)]
pub struct DefaultInnerValueSelf {}
// impl Ord for DefaultInnerValueSelf {
//     fn cmp(&self, _: &Self) -> std::cmp::Ordering {
//         std::cmp::Ordering::Equal
//     }
// }
// // impl PartialEq for InnerValueSelf {
// //     fn eq(&self, other: &Self) -> bool {
// //         true
// //     }
// // }
// // impl Eq for InnerValueSelf {}
// impl PartialOrd for DefaultInnerValueSelf {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(std::cmp::Ordering::Equal)
//     }
// }

impl InnerCodecValue for DefaultInnerValueSelf {
    type Inner = Value<Self>;
}

// impl<V: Eq> Ord for ContainerValue<V> {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         match (self, other) {
//             (Self::Vec(_), Self::Map(_)) => std::cmp::Ordering::Less,
//             (Self::Map(_), Self::Vec(_)) => std::cmp::Ordering::Greater,
//             _ => std::cmp::Ordering::Equal,
//         }
//     }
// }

// impl<V: Eq> PartialOrd for ContainerValue<V> {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

/// A full value: either a primitive [`SimpleValue`] or a [`ContainerValue`]
///
/// If you wanna make your own wrapper of this type follow this:
///
/// ``` no_run
/// pub struct YourValueWrapper {
///     value: Value<YourValueInner>,
///     ANYTHING: ..
/// }
///
/// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// pub struct YourValueInner {}
/// impl InnerCodecValue for YourValueInner {
///     type Inner = YourValueWrapper;
/// }
/// ```
///
/// TODO:
/// Add support for the following:
/// - yaml anchors
/// - hcl expressions
pub enum Value<W: InnerCodecValue> {
    /// [`SimpleValue`]
    Simple(SimpleValue),
    /// [`ContainerValue`]
    Container(ContainerValue<W::Inner>),
}
impl<W: InnerCodecValue> std::fmt::Debug for Value<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple(val) => f.debug_struct("Value").field("Simple", val).finish(),
            Self::Container(val) => f.debug_struct("Value").field("Container", val).finish(),
        }
    }
}
impl<W: InnerCodecValue> Clone for Value<W> {
    fn clone(&self) -> Self {
        match self {
            Self::Container(val) => Self::Container(val.clone()),
            Self::Simple(val) => Self::Simple(val.clone()),
        }
    }
}
impl<W: InnerCodecValue> PartialEq for Value<W> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Simple(a), Self::Simple(b)) => a == b,
            (Self::Container(a), Self::Container(b)) => a == b,
            _ => false,
        }
    }
}
impl<W: InnerCodecValue> Eq for Value<W> {}
impl<W: crate::values::value::InnerCodecValue> From<SimpleValue> for Value<W> {
    fn from(value: SimpleValue) -> Self {
        Self::Simple(value)
    }
}
impl<W: crate::values::value::InnerCodecValue> From<ContainerValue<W::Inner>> for Value<W> {
    fn from(value: ContainerValue<W::Inner>) -> Self {
        Self::Container(value)
    }
}
impl<W: crate::values::value::InnerCodecValue> FromPatch<SimpleValue> for Value<W> {
    fn from_value(value: SimpleValue) -> Self {
        Self::from(value)
    }
}
impl<W: crate::values::value::InnerCodecValue> FromPatch<ContainerValue<W::Inner>> for Value<W> {
    fn from_value(value: ContainerValue<W::Inner>) -> Self {
        Self::from(value)
    }
}

// ── Ord / PartialOrd for ContainerValue ──────────────────────────────────────

// impl<V: Ord> PartialOrd for ContainerValue<V> {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl<V: Ord> Ord for ContainerValue<V> {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         const fn rank<V>(v: &ContainerValue<V>) -> u8 {
//             match v {
//                 ContainerValue::Vec(_) => 0,
//                 ContainerValue::Map(_) => 1,
//             }
//         }

//         match rank(self).cmp(&rank(other)) {
//             std::cmp::Ordering::Equal => {}
//             ord => return ord,
//         }

//         match (self, other) {
//             (Self::Vec(a), Self::Vec(b)) => a.cmp(b),
//             (Self::Map(a), Self::Map(b)) => {
//                 let mut a_items: Vec<_> = a.iter().collect();
//                 let mut b_items: Vec<_> = b.iter().collect();
//                 a_items.sort_by(|x, y| x.0.cmp(y.0));
//                 b_items.sort_by(|x, y| x.0.cmp(y.0));
//                 a_items.cmp(&b_items)
//             }
//             _ => unreachable!(),
//         }
//     }
// }

// ── Ord / PartialOrd for Value ────────────────────────────────────────────────

impl<W: crate::values::value::InnerCodecValue> PartialOrd for Value<W>
where
    <W as crate::values::value::InnerCodecValue>::Inner: std::cmp::Eq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<W: crate::values::value::InnerCodecValue> Ord for Value<W>
// where
//     W::Inner: std::hash::Hash,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Simples sort before containers; within each arm delegate.
        match (self, other) {
            (Self::Simple(a), Self::Simple(b)) => a.cmp(b),
            (Self::Simple(_), Self::Container(_)) => std::cmp::Ordering::Less,
            (Self::Container(_), Self::Simple(_)) => std::cmp::Ordering::Greater,
            (Self::Container(a), Self::Container(b)) => a.cmp(b),
        }
    }
}

// ── Hash for Value ────────────────────────────────────────────────────────────

impl<W: crate::values::value::InnerCodecValue> std::hash::Hash for Value<W>
where
    <W as crate::values::value::InnerCodecValue>::Inner: std::hash::Hash + Ord,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Self::Simple(s) => s.hash(state),
            Self::Container(c) => c.hash(state),
        }
    }
}
use crate::settings::MapType;

// ── Minimal surface the implementor must provide ──────────────────────────────

// ── impl for Value ────────────────────────────────────────────────────────────

impl<W: crate::values::value::InnerCodecValue> CodecContainerSubValueRef for Value<W> {
    type InnerValue = W;

    fn as_container(&self) -> Option<&ContainerValue<W::Inner>> {
        if let Self::Container(c) = self {
            Some(c)
        } else {
            None
        }
    }
}
impl<W: crate::values::value::InnerCodecValue> CodecSimpleSubValueRef for Value<W> {
    fn as_simple(&self) -> Option<&SimpleValue> {
        if let Self::Simple(s) = self {
            Some(s)
        } else {
            None
        }
    }
}

impl<W: crate::values::value::InnerCodecValue> CodecContainerSubValueRefMut for Value<W> {
    type InnerValue = W;

    fn as_container_mut(&mut self) -> Option<&mut ContainerValue<W::Inner>> {
        if let Self::Container(c) = self {
            Some(c)
        } else {
            None
        }
    }
}
impl<W: crate::values::value::InnerCodecValue> CodecSimpleSubValueRefMut for Value<W> {
    fn as_simple_mut(&mut self) -> Option<&mut SimpleValue> {
        if let Self::Simple(c) = self {
            Some(c)
        } else {
            None
        }
    }
}

impl<W: crate::values::value::InnerCodecValue> CodecSimpleSubValueInto for Value<W> {
    fn into_simple(self) -> Option<SimpleValue> {
        if let Self::Simple(s) = self {
            Some(s)
        } else {
            None
        }
    }
}

impl<W: crate::values::value::InnerCodecValue> CodecContainerSubValueInto for Value<W> {
    type InnerValue = W;

    fn into_container(self) -> Option<ContainerValue<W::Inner>> {
        if let Self::Container(c) = self {
            Some(c)
        } else {
            None
        }
    }
}

// Value now gets ValueAccessRef + ValueAccessInto for free via the blanket impls.
impl<V: InnerCodecValue> Value<V>
where
    V::Inner: ToString,
{
    // #[must_use]
    // /// Turns Map<Value, Value> into Map<&String, &Value>
    // pub fn to_map_with_only_string_key_ref(
    //     &self,
    // ) -> Option<crate::settings::MapType<&String, &Self>> {
    //     let map = self.as_map_ref()?;
    //     let mut new = MapType::new();
    //     for (value, item) in map {
    //         let key = value.as_string_ref()?;
    //         new.insert(key, item);
    //     }
    //     Some(new)
    // }
    #[must_use]
    /// Turns Map<Value, Value> into Map<&String, &Value>
    pub fn to_map_with_only_string_key(self) -> Option<crate::settings::MapType<String, V::Inner>> {
        let map = self.into_map()?;
        let mut new = MapType::new();
        for (value, item) in map {
            let key = value.to_string();
            new.insert(key, item);
        }
        Some(new)
    }
}

// // TODO: Uncomment when rust trait resolver is smarter
// impl<
//     T: TryIntoPatch<i128>
//         + TryIntoPatch<f64>
//         + TryIntoPatch<num_bigint::BigInt>
//         + TryIntoPatch<num_bigfloat::BigFloat>
//         + Clone,
// > TryFromPatch<T> for Number
// {
//     fn try_from_value(value: T) -> Option<Self> {
//         value.clone().try_into_value().map_or_else(
//             || {
//                 value.clone().try_into_value().map_or_else(
//                     || value.try_into_value().map(Self::BigFloat),
//                     |value| Some(Self::BigInt(value)),
//                 )
//             },
//             |value| Some(Self::Int(value)),
//         )
//     }
// }
// impl<
//     T: TryFromPatch<i128>
//         + TryFromPatch<f64>
//         + TryFromPatch<num_bigint::BigInt>
//         + TryFromPatch<num_bigfloat::BigFloat>
//         + Clone
//         // + Sealed,
// > TryIntoPatch<T> for Number
// {
//     fn try_into_value(value: Number) -> Option<T> {
//         match value {
//             Number::Int(v) => T::try_from_value(v),
//             Number::Float(v) => T::try_from_value(v),
//             Number::BigFloat(v) => T::try_from_value(v),
//             Number::BigInt(v) => T::try_from_value(v),
//         }
//     }
// }
// trait Sealed {}
// impl_empty_trait!(Sealed for u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize);

// const _: () = {
//     <u8 as TryIntoPatch<i128>>::try_into_value;
//     <u8 as TryIntoPatch<f64>>::try_into_value;
//     <u8 as TryIntoPatch<num_bigint::BigInt>>::try_into_value;
//     <u8 as TryIntoPatch<num_bigfloat::BigFloat>>::try_into_value;
//     <u8 as Clone>::clone;
// };

// /// Create new
// impl<W: InnerCodecValue> Value<W> {
//     #[must_use]
//     /// Creates a new [Value] of [`Value::String`] type
//     pub fn from_string(value: String) -> Self {
//         SimpleValue::String(value).into()
//     }
//     #[must_use]
//     /// Creates a new [Value] of [`Value::Vec`] type
//     pub fn from_vec(value: Vec<W::Inner>) -> Self {
//         ContainerValue::Vec(value).into()
//     }
//     #[must_use]
//     /// Creates a new [Value] of [`Value::Map`] type
//     pub fn from_map(
//         map: std::collections::BTreeMap<W::Inner, W::Inner>,
//     ) -> Self {
//         ContainerValue::Map(map).into()
//     }
//     #[must_use]
//     /// Creates a new [Value] of [`Value::Number`] type
//     pub fn from_number<T: IntoPatch<Number>>(number: T) -> Self {
//         SimpleValue::Number(number.into_value()).into()
//     }
//     #[must_use]
//     /// Creates a new [Value] of [`Value::Number`] type
//     pub fn from_number_legacy<T: Into<Number>>(number: T) -> Self {
//         SimpleValue::Number(number.into()).into()
//     }
//     #[must_use]
//     /// Creates a new [Value] of [`Value::Bool`] type
//     pub fn from_bool(bool: bool) -> Self {
//         SimpleValue::Bool(bool).into()
//     }
// }

impl<W: InnerCodecValue> Value<W> {
    #[must_use]
    /// Creates a new [Value] of [`Value::String`] type from an existing string
    pub fn from_string(value: String) -> Self {
        SimpleValue::String(value).into()
    }

    #[must_use]
    /// Creates an empty [Value] of [`Value::String`] type
    pub fn new_string() -> Self {
        SimpleValue::String(String::new()).into()
    }

    #[must_use]
    /// Creates a new [Value] of [`Value::Vec`] type from an existing vec
    pub fn from_vec(value: Vec<W::Inner>) -> Self {
        ContainerValue::Vec(value).into()
    }

    #[must_use]
    /// Creates an empty [Value] of [`Value::Vec`] type
    pub fn new_vec() -> Self {
        ContainerValue::Vec(Vec::new()).into()
    }

    #[must_use]
    /// Creates a new [Value] of [`Value::Map`] type from an existing map
    pub fn from_map(map: MapType<W::Inner, W::Inner>) -> Self {
        ContainerValue::Map(map).into()
    }

    #[must_use]
    /// Creates an empty [Value] of [`Value::Map`] type
    pub fn new_map() -> Self {
        ContainerValue::Map(MapType::new()).into()
    }

    #[must_use]
    /// Creates a new [Value] of [`Value::None`] type
    pub fn new_none() -> Self {
        SimpleValue::None.into()
    }

    #[must_use]
    /// Creates a new [Value] of [`Value::Number`] type from an existing number
    pub fn from_number<T: IntoPatch<Number>>(number: T) -> Self {
        SimpleValue::Number(number.into_value()).into()
    }

    #[must_use]
    /// Creates an empty (zero) [Value] of [`Value::Number`] type
    pub fn new_number() -> Self {
        SimpleValue::Number(Number::default()).into()
    }

    #[must_use]
    /// Creates a new [Value] of [`Value::Number`] type from an existing number (legacy)
    pub fn from_number_legacy<T: Into<Number>>(number: T) -> Self {
        SimpleValue::Number(number.into()).into()
    }

    #[must_use]
    /// Creates a new [Value] of [`Value::Bool`] type from an existing bool
    pub fn from_bool(bool: bool) -> Self {
        SimpleValue::Bool(bool).into()
    }

    #[must_use]
    /// Creates a new [Value] of [`Value::Bool`] type defaulting to false
    pub fn new_bool() -> Self {
        SimpleValue::Bool(false).into()
    }
}

/// Compatibility
impl<W: InnerCodecValue> Value<W> {
    #[must_use]
    /// Get the value as a Vec if it's a Vec
    ///
    /// Note that this function will not be removed for compatibility reasons
    #[deprecated(note = "It is recommended to use [`as_vec_ref`](Self::as_vec_ref)")]
    pub fn as_array(&self) -> Option<&Vec<W::Inner>> {
        self.as_vec_ref()
    }
    #[must_use]
    /// Get the value as a String if it's a String
    ///
    /// Note that this function will not be removed for compatibility reasons
    #[deprecated(note = "It is recommended to use [`as_string_ref`](Self::as_string_ref)")]
    pub fn as_str(&self) -> Option<&String> {
        self.as_string_ref()
    }
    #[must_use]
    /// Get the value as a String if it's a String
    ///
    /// Note that this function will not be removed for compatibility reasons
    #[deprecated(note = "It is recommended to use [`as_map_ref`](Self::as_map_ref)")]
    pub fn as_object(&self) -> Option<&MapType<W::Inner, W::Inner>> {
        self.as_map_ref()
    }
}
// impl<W: InnerCodecValue: mirl_extensions::Map<W::Inner, W::Inner>>
//     mirl_extensions::GetCodecValueType for Value<W>
// {
// fn get_value_type(&self) -> ValueType {

// }
// }

impl<W: InnerCodecValue> Value<W> {
    #[must_use]
    /// If the given value is of a specific type
    pub fn is_type(&self, value_type: ValueType) -> bool {
        value_type == self.get_value_type()
    }
    #[must_use]
    /// Checks if the given value is of type [`ValueType::Map`]
    pub fn is_map(&self) -> bool {
        self.is_type(ValueType::Map)
    }
    #[must_use]
    /// Checks if the given value is of type [`ValueType::String`]
    pub fn is_string(&self) -> bool {
        self.is_type(ValueType::String)
    }
    #[must_use]
    /// Checks if the given value is of type [`ValueType::Vec`]
    pub fn is_vec(&self) -> bool {
        self.is_type(ValueType::Vec)
    }
    #[must_use]
    /// Checks if the given value is of type [`ValueType::Number`]
    pub fn is_number(&self) -> bool {
        self.is_type(ValueType::Number)
    }
    #[must_use]
    /// Checks if the given value is of type [`ValueType::None`]
    pub fn is_none(&self) -> bool {
        self.is_type(ValueType::None)
    }
    #[must_use]
    /// Checks if the given value is of type [`ValueType::Bool`]
    pub fn is_bool(&self) -> bool {
        self.is_type(ValueType::Bool)
    }
}

impl Value<DefaultInnerValueSelf> {
    #[must_use]
    /// Turn the current type into a vec of size 1
    /// Vec: Returns as is
    /// Map: Returns keys
    /// Other: Returns Vec<&Self>
    pub fn to_list(&self) -> Vec<&Self> {
        match self {
            Self::Simple(_) => {
                vec![self]
            }
            Self::Container(con) => match con {
                ContainerValue::Vec(vec) => vec.iter().collect(),
                ContainerValue::Map(map) => map.keys().collect(),
            },
        }
    }
    #[must_use]
    /// Turn the current type into a vec
    /// Vec: Returns as is
    /// Map: Returns keys
    /// Defined: Returns Vec<&Self>
    /// Other: None
    pub fn to_list_of_type(&self, value_type: ValueType) -> Option<Vec<&Self>> {
        match self {
            Self::Container(con) => match con {
                ContainerValue::Map(map) => {
                    let keys: Vec<&Self> = map.keys().collect();
                    for i in &keys {
                        if !i.is_type(value_type) {
                            return None;
                        }
                    }
                    Some(keys)
                }

                ContainerValue::Vec(val) => {
                    let list: Vec<&Self> = val.iter().collect();
                    for i in &list {
                        if !i.is_type(value_type) {
                            return None;
                        }
                    }
                    Some(list)
                }
            },
            Self::Simple(_) => {
                if self.is_type(value_type) {
                    Some(vec![self])
                } else {
                    None
                }
            }
        }
    }
}

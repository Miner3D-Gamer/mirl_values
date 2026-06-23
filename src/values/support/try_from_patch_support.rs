use mirl_extensions::*;

use crate::{Value, values::value::InnerCodecValue};
impl<W: InnerCodecValue> TryFromPatch<Value<W>> for std::string::String {
    fn try_from_value(value: Value<W>) -> Option<Self> {
        value.into_string()
    }
}
impl<T: TryFromPatch<W::Inner>, W: InnerCodecValue> TryFromPatch<Value<W>> for Vec<T> {
    fn try_from_value(value: Value<W>) -> Option<Self> {
        value.into_vec().and_then(|list| {
            let mut output = Self::with_capacity(list.capacity());
            for i in list {
                output.push(T::try_from_value(i)?);
            }
            Some(output)
        })
    }
}
impl<W: InnerCodecValue> TryFromPatch<Value<W>> for bool {
    fn try_from_value(value: Value<W>) -> Option<Self> {
        value.into_bool()
    }
}

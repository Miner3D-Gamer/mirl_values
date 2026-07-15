use mirl_values_core::value::{ContainerValue, SimpleValue};

use crate::{
    settings::MapType,
    values::{
        Number,
        value::{InnerCodecValue, Value},
    },
};

impl<W: InnerCodecValue> From<toml::Value> for Value<W>
where
    W::Inner: From<Self> + Ord + std::hash::Hash,
{
    fn from(value: toml::Value) -> Self {
        match value {
            toml::Value::Array(vec) => ContainerValue::Vec({
                let mut output = Vec::with_capacity(vec.len());
                for i in vec {
                    output.push(W::Inner::from(Self::from(i)));
                }
                output
            })
            .into(),
            toml::Value::Boolean(bool) => SimpleValue::Bool(bool).into(),
            toml::Value::String(str) => SimpleValue::String(str).into(),
            toml::Value::Datetime(datetime) => SimpleValue::DateTime(datetime.into()).into(),
            toml::Value::Float(float) => SimpleValue::Number(Number::Float(float)).into(),
            toml::Value::Integer(int) => SimpleValue::Number(Number::Int(i128::from(int))).into(),
            toml::Value::Table(table) => ContainerValue::Map({
                let mut tree: MapType<W::Inner, W::Inner> = MapType::new();
                for i in table {
                    tree.insert(
                        W::Inner::from(SimpleValue::String(i.0).into()),
                        W::Inner::from(Self::from(i.1)),
                    );
                }
                tree
            })
            .into(),
        }
    }
}

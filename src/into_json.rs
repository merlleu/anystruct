use prost_types::{Value, Struct};

pub trait IntoJSON {
    fn into_json(self) -> serde_json::Value;
}

impl IntoJSON for Value {
    fn into_json(self) -> serde_json::Value {
        match self.kind {
            Some(kind) => match kind {
                prost_types::value::Kind::NullValue(_) => serde_json::Value::Null,
                prost_types::value::Kind::NumberValue(n) => serde_json::Value::Number(serde_json::Number::from_f64(n).unwrap()),
                prost_types::value::Kind::StringValue(s) => serde_json::Value::String(s),
                prost_types::value::Kind::BoolValue(b) => serde_json::Value::Bool(b),
                prost_types::value::Kind::StructValue(struct_value) => {
                    serde_json::Value::Object(
                        struct_value.fields.into_iter()
                            .map(|(name, val)| (name, val.into_json()))
                            .collect()
                    )
                },
                prost_types::value::Kind::ListValue(list_value) => {
                    serde_json::Value::Array(
                        list_value.values.into_iter()
                            .map(|value| value.into_json())
                            .collect()
                    )
                }
            },
            None => serde_json::Value::Null,
        }
    }
}


pub trait IntoJSONStruct {
    fn into_json_struct(self) -> serde_json::Map<String, serde_json::Value>;
}

impl IntoJSONStruct for Struct {
    fn into_json_struct(self) -> serde_json::Map<String, serde_json::Value> {
        self.fields.into_iter()
            .map(|(name, value)| (name, value.into_json()))
            .collect()
    }
}
use crate::{JsonStruct, ProtoStruct};

pub trait IntoProto {
    fn into_proto(self) -> prost_types::Value;
}

impl IntoProto for serde_json::Value {
    fn into_proto(self) -> prost_types::Value {
        match self {
            serde_json::Value::Null => prost_types::Value { kind: None },
            serde_json::Value::Bool(b) => prost_types::Value {
                kind: Some(prost_types::value::Kind::BoolValue(b)),
            },
            serde_json::Value::Number(n) => prost_types::Value {
                kind: Some(prost_types::value::Kind::NumberValue(
                    n.as_f64().unwrap_or_default(),
                )),
            },
            serde_json::Value::String(s) => prost_types::Value {
                kind: Some(prost_types::value::Kind::StringValue(s)),
            },
            serde_json::Value::Array(a) => prost_types::Value {
                kind: Some(prost_types::value::Kind::ListValue(
                    prost_types::ListValue {
                        values: a.into_iter().map(|v| v.into_proto()).collect(),
                    },
                )),
            },
            serde_json::Value::Object(o) => prost_types::Value {
                kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
                    fields: o.into_iter().map(|(k, v)| (k, v.into_proto())).collect(),
                })),
            },
        }
    }
}

pub trait IntoProtoStruct {
    fn into_proto_struct(self) -> prost_types::Struct;
}

impl IntoProtoStruct for JsonStruct {
    fn into_proto_struct(self) -> ProtoStruct {
        let fields = self
            .into_iter()
            .map(|(name, value)| {
                let proto_value = value.into_proto();
                (name, proto_value)
            })
            .collect();

        ProtoStruct { fields }
    }
}
mod into_json;
mod into_proto;

pub use into_json::{IntoJSON, IntoJSONStruct};
pub use into_proto::{IntoProto, IntoProtoStruct};

pub type JsonStruct = serde_json::Map<String, serde_json::Value>;
pub type ProtoStruct = prost_types::Struct;
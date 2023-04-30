mod into_json;
mod into_proto;

use std::collections::HashMap;

pub use into_json::{IntoJSON, IntoJSONStruct};
pub use into_proto::{IntoProto, IntoProtoStruct};

pub type JsonStruct = HashMap<String, serde_json::Value>;
pub type ProtoStruct = prost_types::Struct;
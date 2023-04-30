use std::collections::BTreeMap;

use anystruct::{IntoJSON, IntoProto, IntoJSONStruct, IntoProtoStruct};
use prost_types::{Struct, Value};
use serde_json::{Map, Value as JsonValue};

#[test]
fn test_json_to_proto_empty() {
    let json_str = r#"{}"#;

    let expected_proto_value = Value { kind: None };

    let json_value: JsonValue = serde_json::from_str(json_str).unwrap();
    let proto_value = json_value.into_proto();
    assert_eq!(proto_value, expected_proto_value);
}

#[test]
fn test_proto_to_json_empty() {
    let proto_value = Value { kind: None };

    let expected_json_value: JsonValue = serde_json::from_str(r#"null"#).unwrap();

    let json_value = proto_value.into_json();
    assert_eq!(json_value, expected_json_value);
}

#[test]
fn test_json_struct_to_proto_empty() {
    let expected_proto_struct = Struct::default();

    let json_map: Map<String, JsonValue> = serde_json::from_str(r#"{}"#).unwrap();
    let proto_struct = json_map.into_proto_struct();
    assert_eq!(proto_struct, expected_proto_struct);
}

#[test]
fn test_proto_to_json_struct_empty() {
    let proto_struct = Struct::default();

    let expected_json_map: Map<String, JsonValue> = serde_json::from_str(r#"{}"#).unwrap();

    let json_map = proto_struct.into_json_struct();
    assert_eq!(json_map, expected_json_map);
}

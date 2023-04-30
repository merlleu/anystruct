# AnyStruct Crate

AnyStruct is a Rust crate that provides traits for converting between JSON and Protocol Buffers (Proto) data structures. Specifically, it offers the following traits:

- `IntoJSON`: Converts a `Value` struct from the `prost_types` crate to a `serde_json::Value` struct.
- `IntoProto`: Converts a `serde_json::Value` struct to a `Value` struct from the `prost_types` crate.
- `IntoJSONStruct`: Converts a `Struct` struct from the `prost_types` crate to a `serde_json::Map` struct.
- `IntoProtoStruct`: Converts a `serde_json::Map` struct to a `Struct` struct from the `prost_types` crate.

## Usage

To use AnyStruct in your Rust project, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
anystruct = "0.1.0"
```

Here is an example usage for converting a JSON string to a Proto struct:

```rust
use anystruct::{IntoProto, ProtoStruct};

let json_str = r#"{
    "name": "John Doe",
    "age": 42,
    "is_student": true,
    "grades": [80, 85, 90],
    "address": {
        "street": "123 Main St",
        "city": "Anytown",
        "state": "CA"
    }
}"#;

let json_value = serde_json::from_str(json_str).unwrap();
let proto_value = json_value.into_proto();
let proto_struct = ProtoStruct { fields: [("my_data".to_string(), proto_value)].iter().cloned().collect() };
```

And here is an example usage for converting a Proto struct to a JSON string:

```rust
use anystruct::{IntoJSON, IntoJSONStruct};

let proto_value = prost_types::Value {
    kind: Some(prost_types::value::Kind::StructValue(prost_types::Struct {
        fields: [("name".to_string(), prost_types::Value {
            kind: Some(prost_types::value::Kind::StringValue("John Doe".to_string())),
        })].iter().cloned().collect(),
    })),
};
let json_value = proto_value.into_json();
let json_map = json_value.as_object().unwrap().clone();
let json_str = serde_json::to_string_pretty(&json_map).unwrap();
```

You can also use the `IntoProtoStruct` and `IntoJSONStruct` traits to convert between `Struct` and `serde_json::Map` structs:

```rust
let proto_struct = json_map.into_proto_struct();
let json_map2 = proto_struct.into_json_struct();
```
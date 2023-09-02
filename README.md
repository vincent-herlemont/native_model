# Native model

A thin wrapper around serialized data which add information of identity and version.

## Goals

- **Interoperability**: Allows different applications to work together, even if they are using different 
  versions of the data model.
- **Data Consistency**: Ensure that we process the data expected model.
- **Flexibility**: You can use any serialization format you want. More details [here](#setup-your-serialization-format).
- **Performance**: A minimal overhead. More details [here](#performance).

## Usage

```
            Application 1 (DotV1)                     Application 2 (DotV1 and DotV2)
                       |                                          |
          Encode DotV1 |----------------------------------------> | Decode DotV1 to DotV2
                       |                                          | Modify DotV2
          Decode DotV1 | <----------------------------------------| Encode DotV2 back to DotV1
                       |                                          |
```


```rust,skt-main
// Application 1
let dot = DotV1(1, 2);
let bytes = native_model::encode(&dot).unwrap();

// Application 1 sends bytes to Application 2.

// Application 2
// We are able to decode the bytes directly into a new type DotV2.
let (mut dot, source_version) = native_model::decode::<DotV2>(bytes).unwrap();
assert_eq!(dot, DotV2 { 
    name: "".to_string(), 
    x: 1, 
    y: 2 
});
dot.name = "Dot".to_string();
dot.x = 5;
// For interoperability, we encode the data with the version compatible with Application 1.
let bytes = native_model::encode_downgrade(dot, source_version).unwrap();

// Application 2 sends bytes to Application 1.

// Application 1
let (dot, _) = native_model::decode::<DotV1>(bytes).unwrap();
assert_eq!(dot, DotV1(5, 2));
 ```

Full example [here](./tests/example/example_main.rs).

When use it?
- All applications that interact with each other are written in Rust.
- Your applications evolve independently need to read serialized data coming from each other.
- Your applications store data locally and need to read it later by a newer version of the application.
- Your systems need to be upgraded incrementally. Instead of having to upgrade the entire system at once, individual
  applications can be upgraded one at a time, while still being able to communicate with each other.

When not use it?
- All applications that interact with each other are **not** written in Rust.
- Applications need to communicate with other systems that you don't control.
- You need to have a human-readable format. (You can use a human-readable format like JSON wrapped in a native model,
  but you have to unwrap it to see the data correctly.)

## Setup your serialization format

First, you need to set up your serialization format. You can use any serialization format.

Just define the following functions, so they must be imported in the scope where you use the native model.

```rust,ignore
fn native_model_encode_body<T: Encode>(obj: &T) -> Result<Vec<u8>, dyn Error> {
   ...
}

fn native_model_decode_body<T: Decode>(data: Vec<u8>) -> Result<T, dyn Error> {
   ...
}
```
Examples: 
- [bincode with encode/decode](./tests/encode_decode/bincode.rs)
- [bincode with serde](./tests/encode_decode/bincode_serde.rs)


## Setup your data model

Define your model using the macro [`native_model`](file:///home/vincentherlemont/IdeaProjects/native_model/target/doc/native_model/attr.native_model.html).

Attributes:
- `id = u32`: The unique identifier of the model.
- `version = u32`: The version of the model.
- `from = type`: Optional, the previous version of the model.
    - `type`: The previous version of the model that you use for the From implementation.
- `try_from = (type, error)`: Optional, the previous version of the model with error handling.
    - `type`: The previous version of the model that you use for the TryFrom implementation.
    - `error`: The error type that you use for the TryFrom implementation.

```rust,skt-define-models
use native_model::native_model;

#[derive(Encode, Decode, PartialEq, Debug)]
#[native_model(id = 1, version = 1)]
struct DotV1(u32, u32);

#[derive(Encode, Decode, PartialEq, Debug)]
#[native_model(id = 1, version = 2, from = DotV1)]
struct DotV2 {
    name: String,
    x: u64,
    y: u64,
}

// Implement the conversion between versions From<DotV1> for DotV2 and From<DotV2> for DotV1.

#[derive(Encode, Decode, PartialEq, Debug)]
#[native_model(id = 1, version = 3, try_from = (DotV2, anyhow::Error))]
struct DotV3 {
    name: String,
    cord: Cord,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct Cord {
    x: u64,
    y: u64,
}

// Implement the conversion between versions From<DotV2> for DotV3 and From<DotV3> for DotV2.
```

Full example [here](tests/example/example_define_model.rs).


# Performance

This crate is in an early stage of development, so the performance should be improved in the future.
The goal is to have a minimal and constant overhead for all data sizes. It uses the [zerocopy](https://docs.
rs/zerocopy/latest/zerocopy/) crate to avoid unnecessary copies.

Current performance:
- Encode time: have overhead that evolves linearly with the data size.
- Decode time: have overhead of ~162 ps for all data sizes.


|       data size       | encode time (ns/ps/µs/ms) | decode time (ps) |
|:---------------------:|:--------------------------:|:----------------:|
|          1 B          | 40.093 ns - 40.510 ns      | 161.87 ps - 162.02 ps |
|    1 KiB (1024 B)     | 116.45 ns - 116.83 ns      | 161.85 ps - 162.08 ps |
|   1 MiB (1048576 B)   | 66.697 µs - 67.634 µs      | 161.87 ps - 162.18 ps |
|  10 MiB (10485760 B)  | 1.5670 ms - 1.5843 ms      | 162.40 ps - 163.52 ps |
| 100 MiB (104857600 B) | 63.778 ms - 64.132 ms      | 162.71 ps - 165.10 ps |

Benchmark of the native model overhead [here](benches/overhead.rs).

To know how much time it takes to encode/decode your data, you need to add this overhead to the time of your serialization format.
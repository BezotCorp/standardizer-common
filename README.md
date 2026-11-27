# Standardizer Common

`standardizer-common` defines the shared data structures, validation rules,
and serialization formats used by Standardizer and its related tooling.

## Features

- `source`: RON deserialization and validation.
- `encode`: binary encoding and validation.
- `decode`: binary decoding and validation.

No serialization feature is enabled by default.

Serialization dependencies and binary configuration are managed by this
library. Consumers do not need direct dependencies on RON, Serde, or bincode
to serialize or deserialize the shared data structures.

## Usage

Both `Routing` and `ToolConfiguration` expose the same operations:

- `from_ron`: deserialize and validate RON data.
- `encode`: validate and encode binary data.
- `decode`: decode and validate binary data.

### Data generation

Enable the `source` and `encode` features.

```rust
use standardizer_common::routes::Routing;

fn generate(source: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Routing::from_ron(source)?.encode()
}
```

### Data consumption

Enable the `decode` feature.

```rust
use standardizer_common::routes::Routing;

fn load(bytes: &[u8]) -> Result<Routing, Box<dyn std::error::Error>> {
    Routing::decode(bytes)
}
```

File access and binary artifact generation remain the responsibility of
consumers.

## Development

```sh
cargo check --all-features
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).

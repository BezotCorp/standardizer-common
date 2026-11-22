# Standardizer Common

`standardizer-common` contains the routing types shared by Standardizer and its
related tooling.

## Features

- `source`: Serde deserialization
- `encode`: bincode encoding
- `decode`: bincode decoding

No serialization feature is enabled by default.

## Usage

```toml
[dependencies]
standardizer-common = "1"
```

## Development

```sh
cargo check --all-features
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).

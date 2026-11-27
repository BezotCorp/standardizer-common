#![cfg(any(feature = "source", feature = "decode"))]

use crate::routes::Routing;

#[test]
#[cfg(feature = "source")]
fn rejects_malformed_ron() {
    assert!(Routing::from_ron("invalid").is_err());
}

#[test]
#[cfg(feature = "source")]
fn rejects_missing_routes() {
    assert!(Routing::from_ron("()").is_err());
}

#[test]
#[cfg(feature = "source")]
fn rejects_empty_extensions() {
    assert!(Routing::from_ron(r#"(routes: [(extensions: [], tools: [Rustfmt])])"#,).is_err());
}

#[test]
#[cfg(feature = "source")]
fn rejects_empty_tools() {
    assert!(Routing::from_ron(r#"(routes: [(extensions: ["rs"], tools: [])])"#,).is_err());
}

#[test]
#[cfg(feature = "source")]
fn rejects_unknown_tool() {
    assert!(Routing::from_ron(r#"(routes: [(extensions: ["rs"], tools: [Unknown])])"#,).is_err());
}

#[test]
#[cfg(feature = "decode")]
fn rejects_empty_binary_input() {
    assert!(Routing::decode(&[]).is_err());
}

#[test]
#[cfg(feature = "decode")]
fn rejects_trailing_bytes() {
    assert!(Routing::decode(&[0, 0]).is_err());
}

#[test]
#[cfg(feature = "decode")]
fn rejects_invalid_binary_data() {
    // One route, no extensions, one Rustfmt tool.
    assert!(Routing::decode(&[1, 0, 1, 0]).is_err());
}

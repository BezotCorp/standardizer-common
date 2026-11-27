#![cfg(any(feature = "source", feature = "decode"))]

use crate::tools::ToolConfiguration;

#[test]
#[cfg(feature = "source")]
fn rejects_malformed_ron() {
    assert!(ToolConfiguration::from_ron("invalid").is_err());
}

#[test]
#[cfg(feature = "source")]
fn rejects_missing_tools() {
    assert!(ToolConfiguration::from_ron("()").is_err());
}

#[test]
#[cfg(feature = "source")]
fn rejects_missing_version() {
    assert!(
        ToolConfiguration::from_ron(r#"(tools: [(id: Biome, minimum_version: "2.6.0")])"#,)
            .is_err()
    );
}

#[test]
#[cfg(feature = "source")]
fn rejects_unknown_tool() {
    assert!(
        ToolConfiguration::from_ron(
            r#"(tools: [(id: Unknown, minimum_version: "2.6.0", managed_version: "2.6.8")])"#,
        )
        .is_err()
    );
}

#[test]
#[cfg(feature = "source")]
fn rejects_duplicate_definitions() {
    let source = r#"(tools: [
        (id: Biome, minimum_version: "2.6.0", managed_version: "2.6.8"),
        (id: Biome, minimum_version: "2.6.0", managed_version: "2.6.8"),
    ])"#;

    assert!(ToolConfiguration::from_ron(source).is_err());
}

#[test]
#[cfg(feature = "decode")]
fn rejects_empty_binary_input() {
    assert!(ToolConfiguration::decode(&[]).is_err());
}

#[test]
#[cfg(feature = "decode")]
fn rejects_trailing_bytes() {
    assert!(ToolConfiguration::decode(&[0, 0]).is_err());
}

#[test]
#[cfg(feature = "decode")]
fn rejects_duplicate_binary_definitions() {
    // Two identical Biome definitions with empty version strings.
    assert!(ToolConfiguration::decode(&[2, 1, 0, 0, 1, 0, 0]).is_err());
}

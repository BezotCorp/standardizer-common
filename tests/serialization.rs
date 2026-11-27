#![cfg(all(feature = "source", feature = "encode", feature = "decode"))]

use standardizer_common::{
    routes::Routing,
    tools::{Tool, ToolConfiguration},
};
use std::{error::Error, path::Path};

#[test]
fn routing_round_trip() -> Result<(), Box<dyn Error>> {
    let source = r#"(routes: [
        (extensions: ["rs"], tools: [Rustfmt]),
        (extensions: ["js", "ts"], tools: [Biome]),
    ])"#;

    let routing = Routing::from_ron(source)?;
    let bytes = routing.encode()?;
    let decoded = Routing::decode(&bytes)?;

    assert_eq!(decoded.routes().len(), 2);
    assert_eq!(decoded.routes()[0].extensions(), &["rs"]);
    assert_eq!(decoded.routes()[0].tools(), &[Tool::Rustfmt]);
    assert_eq!(decoded.routes()[1].extensions(), &["js", "ts"]);
    assert_eq!(decoded.routes()[1].tools(), &[Tool::Biome]);

    assert_eq!(
        decoded.tools(Path::new("main.rs")),
        [Tool::Rustfmt].into_iter().collect(),
    );
    assert_eq!(
        decoded.tools(Path::new("main.RS")),
        [Tool::Rustfmt].into_iter().collect(),
    );
    assert_eq!(
        decoded.tools(Path::new("main.ts")),
        [Tool::Biome].into_iter().collect(),
    );
    assert!(decoded.tools(Path::new("unknown.txt")).is_empty());

    assert_eq!(decoded.encode()?, bytes);

    Ok(())
}

#[test]
fn tool_configuration_round_trip() -> Result<(), Box<dyn Error>> {
    let source = r#"(tools: [
        (id: Rustfmt, minimum_version: "1.0.0", managed_version: "1.1.0"),
        (id: Biome, minimum_version: "2.6.0", managed_version: "2.6.8"),
    ])"#;

    let configuration = ToolConfiguration::from_ron(source)?;
    let bytes = configuration.encode()?;
    let decoded = ToolConfiguration::decode(&bytes)?;

    assert_eq!(decoded.tools().len(), 2);

    let rustfmt = decoded
        .definition(Tool::Rustfmt)
        .ok_or("missing Rustfmt definition")?;

    assert_eq!(rustfmt.id(), Tool::Rustfmt);
    assert_eq!(rustfmt.minimum_version(), "1.0.0");
    assert_eq!(rustfmt.managed_version(), "1.1.0");

    let biome = decoded
        .definition(Tool::Biome)
        .ok_or("missing Biome definition")?;

    assert_eq!(biome.id(), Tool::Biome);
    assert_eq!(biome.minimum_version(), "2.6.0");
    assert_eq!(biome.managed_version(), "2.6.8");

    assert_eq!(decoded.encode()?, bytes);

    Ok(())
}

#[test]
fn routing_binary_format() -> Result<(), Box<dyn Error>> {
    let routing = Routing::from_ron(r#"(routes: [(extensions: ["rs"], tools: [Rustfmt])])"#)?;

    // One route, one extension, two UTF-8 bytes, one tool,
    // Rustfmt enum discriminant.
    let expected = [1, 1, 2, b'r', b's', 1, 0];

    assert_eq!(routing.encode()?, expected);

    let decoded = Routing::decode(&expected)?;

    assert_eq!(decoded.encode()?, expected);

    Ok(())
}

#[test]
fn tool_configuration_binary_format() -> Result<(), Box<dyn Error>> {
    let configuration = ToolConfiguration::from_ron(
        r#"(tools: [(id: Biome, minimum_version: "2.6.0", managed_version: "2.6.8")])"#,
    )?;

    // One definition, Biome enum discriminant,
    // minimum version and managed version.
    let expected = [
        1, 1, 5, b'2', b'.', b'6', b'.', b'0', 5, b'2', b'.', b'6', b'.', b'8',
    ];

    assert_eq!(configuration.encode()?, expected);

    let decoded = ToolConfiguration::decode(&expected)?;

    assert_eq!(decoded.encode()?, expected);

    Ok(())
}

#[test]
fn routing_rejects_every_truncated_binary() -> Result<(), Box<dyn Error>> {
    let routing = Routing::from_ron(r#"(routes: [(extensions: ["rs"], tools: [Rustfmt])])"#)?;

    let bytes = routing.encode()?;

    for length in 0..bytes.len() {
        assert!(
            Routing::decode(&bytes[..length]).is_err(),
            "accepted truncated routing at length {length}",
        );
    }

    Ok(())
}

#[test]
fn tool_configuration_rejects_every_truncated_binary() -> Result<(), Box<dyn Error>> {
    let configuration = ToolConfiguration::from_ron(
        r#"(tools: [(id: Biome, minimum_version: "2.6.0", managed_version: "2.6.8")])"#,
    )?;

    let bytes = configuration.encode()?;

    for length in 0..bytes.len() {
        assert!(
            ToolConfiguration::decode(&bytes[..length]).is_err(),
            "accepted truncated configuration at length {length}",
        );
    }

    Ok(())
}

#[test]
fn empty_datasets_round_trip() -> Result<(), Box<dyn Error>> {
    let routing = Routing::from_ron("(routes: [])")?;
    let configuration = ToolConfiguration::from_ron("(tools: [])")?;

    let routing_bytes = routing.encode()?;
    let configuration_bytes = configuration.encode()?;

    assert_eq!(routing_bytes, [0]);
    assert_eq!(configuration_bytes, [0]);

    assert!(Routing::decode(&routing_bytes)?.routes().is_empty());
    assert!(
        ToolConfiguration::decode(&configuration_bytes)?
            .tools()
            .is_empty()
    );

    Ok(())
}

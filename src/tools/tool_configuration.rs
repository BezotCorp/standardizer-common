use crate::tools::{Tool, ToolDefinition};
use std::collections::BTreeSet;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "source", derive(serde::Deserialize))]
#[cfg_attr(feature = "encode", derive(bincode_next::Encode))]
#[cfg_attr(feature = "decode", derive(bincode_next::Decode))]
pub struct ToolConfiguration {
    tools: Vec<ToolDefinition>,
}

impl ToolConfiguration {
    /// Deserializes and validates RON data.
    #[cfg(feature = "source")]
    pub fn from_ron(source: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let value: Self = ron::from_str(source)?;
        value.validate()?;
        Ok(value)
    }

    /// Validates and encodes data using the shared binary configuration.
    #[cfg(feature = "encode")]
    pub fn encode(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.validate()?;
        Ok(bincode_next::encode_to_vec(
            self,
            bincode_next::config::standard(),
        )?)
    }

    /// Decodes and validates data, rejecting trailing bytes.
    #[cfg(feature = "decode")]
    pub fn decode(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let (value, consumed): (Self, usize) =
            bincode_next::decode_from_slice(bytes, bincode_next::config::standard())?;

        if consumed != bytes.len() {
            return Err("trailing data in binary input".into());
        }

        value.validate()?;
        Ok(value)
    }

    pub fn definition(&self, id: Tool) -> Option<&ToolDefinition> {
        self.tools.iter().find(|definition| definition.id() == id)
    }

    pub fn tools(&self) -> &[ToolDefinition] {
        &self.tools
    }

    pub fn validate(&self) -> Result<(), String> {
        let mut ids = BTreeSet::new();

        for definition in &self.tools {
            if !ids.insert(definition.id()) {
                return Err(format!(
                    "duplicate tool definition for {}",
                    definition.id().name(),
                ));
            }
        }

        Ok(())
    }
}

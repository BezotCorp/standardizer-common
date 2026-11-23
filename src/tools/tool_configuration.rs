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

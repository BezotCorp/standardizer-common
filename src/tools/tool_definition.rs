use crate::tools::Tool;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "source", derive(serde::Deserialize))]
#[cfg_attr(feature = "encode", derive(bincode_next::Encode))]
#[cfg_attr(feature = "decode", derive(bincode_next::Decode))]
pub struct ToolDefinition {
    id: Tool,
    minimum_version: String,
    managed_version: String,
}

impl ToolDefinition {
    pub fn id(&self) -> Tool {
        self.id
    }

    pub fn minimum_version(&self) -> &str {
        &self.minimum_version
    }

    pub fn managed_version(&self) -> &str {
        &self.managed_version
    }
}

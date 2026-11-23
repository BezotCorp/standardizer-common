use crate::tools::Tool;

#[cfg_attr(feature = "source", derive(serde::Deserialize))]
#[cfg_attr(feature = "encode", derive(bincode_next::Encode))]
#[cfg_attr(feature = "decode", derive(bincode_next::Decode))]
pub struct Route {
    extensions: Vec<String>,
    tools: Vec<Tool>,
}

impl Route {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.extensions.is_empty() || self.tools.is_empty() {
            return Err("routing rules require extensions and tools");
        }
        Ok(())
    }

    pub fn extensions(&self) -> &[String] {
        &self.extensions
    }

    pub fn tools(&self) -> &[Tool] {
        &self.tools
    }
}

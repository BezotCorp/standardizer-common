use crate::{routes::Route, tools::Tool};
use std::{collections::BTreeSet, path::Path};

#[derive(Debug)]
#[cfg_attr(feature = "source", derive(serde::Deserialize))]
#[cfg_attr(feature = "encode", derive(bincode_next::Encode))]
#[cfg_attr(feature = "decode", derive(bincode_next::Decode))]
pub struct Routing {
    routes: Vec<Route>,
}

impl Routing {
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

    pub fn validate(&self) -> Result<(), &'static str> {
        self.routes.iter().try_for_each(Route::validate)
    }

    pub fn routes(&self) -> &[Route] {
        &self.routes
    }

    /// Tools to run for `path`, selected by its extension (case-insensitive).
    pub fn tools(&self, path: &Path) -> BTreeSet<Tool> {
        let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
            return BTreeSet::new();
        };

        self.routes
            .iter()
            .filter(|route| {
                route
                    .extensions()
                    .iter()
                    .any(|candidate| candidate.eq_ignore_ascii_case(extension))
            })
            .flat_map(|route| route.tools().iter().copied())
            .collect()
    }
}

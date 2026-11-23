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

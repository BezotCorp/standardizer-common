#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Package {
    name: &'static str,
    version: &'static str,
    description: &'static str,
    repository: &'static str,
    license: &'static str,
}

impl Package {
    pub const fn current() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            description: env!("CARGO_PKG_DESCRIPTION"),
            repository: env!("CARGO_PKG_REPOSITORY"),
            license: env!("CARGO_PKG_LICENSE"),
        }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn version(&self) -> &'static str {
        self.version
    }

    pub const fn description(&self) -> &'static str {
        self.description
    }

    pub const fn repository(&self) -> &'static str {
        self.repository
    }

    pub const fn license(&self) -> &'static str {
        self.license
    }
}

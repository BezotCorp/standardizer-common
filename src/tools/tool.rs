#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "source", derive(serde::Deserialize))]
#[cfg_attr(feature = "encode", derive(bincode_next::Encode))]
#[cfg_attr(feature = "decode", derive(bincode_next::Decode))]
pub enum Tool {
    Rustfmt,
    Biome,
}

impl Tool {
    pub fn name(self) -> &'static str {
        match self {
            Self::Rustfmt => "rustfmt",
            Self::Biome => "biome",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Rustfmt => "Rustfmt",
            Self::Biome => "Biome",
        }
    }
}

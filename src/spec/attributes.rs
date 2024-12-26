use enum_map::Enum;
use serde::{Serialize, Deserialize};
use strum_macros::{EnumIter, EnumString, IntoStaticStr};

#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq, EnumIter, IntoStaticStr, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "kebab-case", ascii_case_insensitive)]
pub enum Attribute {
    Culture,
    Economy,
    Loyalty,
    Stability,
}

#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq, EnumIter, IntoStaticStr, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "kebab-case", ascii_case_insensitive)]
pub enum Ruin {
    Corruption,  // opposes Culture
    Crime,       // opposes Economy
    Strife,      // opposes Loyalty
    Decay,       // opposes Stability
}

impl Attribute {
    pub fn to_markdown(self) -> &'static str {
        self.into()
    }
}

impl Ruin {
    pub fn to_markdown(self) -> &'static str {
        self.into()
    }
}

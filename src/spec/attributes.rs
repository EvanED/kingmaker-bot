use enum_map::Enum;
use serde::{Serialize, Deserialize};
use strum_macros::{EnumString, IntoStaticStr, EnumIter};

#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq, EnumIter, IntoStaticStr, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "kebab-case", ascii_case_insensitive)]
pub enum Attribute {
    Culture,
    Economy,
    Loyalty,
    Stability,
}

impl Attribute {
    pub fn to_markdown(self) -> &'static str {
        self.into()
    }
}

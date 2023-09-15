use enum_map::Enum;
use strum_macros::{EnumString, IntoStaticStr};

#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq, IntoStaticStr, EnumString)]
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

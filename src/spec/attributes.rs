use enum_map::Enum;
use strum_macros::EnumString;

#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "kebab-case", ascii_case_insensitive)]
pub enum Attribute {
    Culture,
    Economy,
    Loyalty,
    Stability,
}

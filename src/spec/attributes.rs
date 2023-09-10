use enum_map::Enum;

#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq)]
pub enum Attribute {
    Culture,
    Economy,
    Loyalty,
    Stability,
}

use enum_map::Enum;

#[derive(Debug, Enum, Clone, Copy)]
pub enum Attribute {
    Culture,
    Economy,
    Loyalty,
    Stability,
}

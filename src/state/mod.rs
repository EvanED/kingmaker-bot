use enum_map::{EnumMap, Enum};
use strum_macros::EnumString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum, EnumString)]
#[strum(serialize_all = "kebab-case", ascii_case_insensitive)]
pub enum Commodity {
    Food,
    Lumber,
    Luxuries,
    Ore,
    Stone,
}

#[derive(Debug, Default, Clone)]
pub struct KingdomState {
    pub unrest: i8,
    pub resource_points: i8,  // More?
    pub fame_points: i8,
    pub commodity_stores: EnumMap<Commodity, i8>,
}

#[cfg(test)]
pub mod tests {
    use assert2::assert;
    use std::str::FromStr;
    use super::*;

    #[test]
    fn blah() {
        assert!(Commodity::Food == Commodity::from_str("food").unwrap());
    }
}

use enum_map::{EnumMap, Enum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
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

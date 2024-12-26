#[derive(Clone,Copy,Debug)]
pub enum TerrainType {
    Plains,
    Hills,
    Forest,
    Swamp,
    Mountains,
}

impl TerrainType {
    pub fn from_string(s: &str) -> Option<TerrainType> {
        use TerrainType::*;
        return match s {
            "plains"    => Some(Plains),
            "hills"     => Some(Hills),
            "forest"    => Some(Forest),
            "swamp"     => Some(Swamp),
            "mountains" => Some(Mountains),
            _           => None,
        }
    }

    pub fn rp_cost(self) -> i8 {
        use TerrainType::*;
        match self {
            Plains    =>  1,
            Hills     =>  2,
            Forest    =>  4,
            Swamp     =>  8,
            Mountains => 12,
        }
    }

    pub fn lowercase_string(self) -> &'static str {
        use TerrainType::*;
        match self {
            Plains    => "plains",
            Hills     => "hills",
            Forest    => "forest",
            Swamp     => "swamp",
            Mountains => "mountains",
        }
    }
}

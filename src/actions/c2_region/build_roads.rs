use crate::{rolls::{bonus, roll_context::RollContext, roll_result::{self, RollResult}}, spec::{skills::Skill, Kingdom}, state::KingdomState, turns::TurnState};

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

pub fn build_roads(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext, terrain: TerrainType) -> (RollResult, TurnState, KingdomState) {
    let the_roll = kingdom.roll(state, bonus::KingdomAction::BuildRoads, Skill::Engineering, context);
    let dc = state.control_dc(kingdom);

    let degree = roll_result::rate_success(
        the_roll.natural,
        the_roll.total,
        dc,
    );

    let roll_result = RollResult {
        die_roll: the_roll,
        degree,
        dc,
    };

    let rp_cost = terrain.rp_cost();

    let mut next_turn_state = turn.clone();
    next_turn_state.requirements.push(
        format!("subtract {rp_cost} RP if there is a river crossing the hex")
    );
    next_turn_state.requirements.push(
        "mark the map with the new roads".to_string()
    );
    next_turn_state.requirements.push(
        format!(
            "can mark an adjacent hex with new roads as well, if it is {} or easier",
            terrain.lowercase_string(),
        )
    );

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.resource_points -= rp_cost;

    (roll_result, next_turn_state, next_kingdom_state)
}
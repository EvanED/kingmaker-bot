use crate::{rolls::{bonus, roll_context::RollContext, roll_result::{self, DegreeOfSuccess, RollResult}}, spec::{skills::Skill, terrain::TerrainType, Kingdom}, state::KingdomState, turns::TurnState};

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
    if degree.passed() {
        next_turn_state.requirements.push(
            "mark the map with the new roads".to_string()
        );
    }
    if degree == DegreeOfSuccess::CriticalSuccess {
        next_turn_state.requirements.push(
            format!(
                "can mark an adjacent hex with new roads as well, if it is {} or easier",
                terrain.lowercase_string(),
            )
        );
    }

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.resource_points -= rp_cost;

    if degree == DegreeOfSuccess::CriticalFailure {
        next_kingdom_state.unrest += 1;
    }

    (roll_result, next_turn_state, next_kingdom_state)
}
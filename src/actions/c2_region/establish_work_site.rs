use crate::{rolls::{bonus, roll_context::RollContext, roll_result::{self, DegreeOfSuccess, RollResult}}, spec::{skills::Skill, terrain::TerrainType, Kingdom}, state::KingdomState, turns::TurnState};

pub fn establish_work_site(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext, terrain: TerrainType) -> (RollResult, TurnState, KingdomState) {
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
    if degree.passed() {
        next_turn_state.requirements.push(
            "mark the map with the new work site".to_string()
        );
        next_turn_state.requirements.push(
            "increase the income for the commodity by 1".to_string()
        );
    }
    if degree == DegreeOfSuccess::CriticalSuccess {
        next_turn_state.requirements.push(
            "there is a bonus commodity income next turn".to_string(),
        );
    }

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.resource_points -= rp_cost;

    if degree == DegreeOfSuccess::CriticalFailure {
        next_kingdom_state.unrest += 1;
    }

    (roll_result, next_turn_state, next_kingdom_state)
}
use crate::{rolls::{bonus, roll_context::RollContext, roll_result::{self, DegreeOfSuccess, RollResult}}, spec::{skills::Skill, Kingdom}, state::{Commodity, KingdomState}, turns::TurnState};

pub fn go_fishing(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext) -> (RollResult, TurnState, KingdomState) {
    let the_roll = kingdom.roll(state, bonus::KingdomAction::GoFishing, Skill::Boating, context);
    let d4 = context.d4.roll();
    let dc = state.control_dc(kingdom);

    let degree = roll_result::rate_success(
        the_roll.natural,
        the_roll.total,
        dc,
    );

    let food_increase = match degree {
        DegreeOfSuccess::CriticalSuccess => d4,
        DegreeOfSuccess::Success         => 1,
        _                                => 0,
    };

    let unrest_increase = match degree {
        DegreeOfSuccess::CriticalFailure => d4,
        _                                => 0,
    };

    let next_turn_state = turn.clone();

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.commodity_stores[Commodity::Food] += food_increase;
    next_kingdom_state.unrest += unrest_increase;

    let roll_result = RollResult {
        die_roll: the_roll,
        degree,
        dc,
    };

    (roll_result, next_turn_state, next_kingdom_state)
}

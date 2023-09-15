use crate::{state::{KingdomState, Commodity}, rolls::{roll_context::RollContext, roll_result::{DC, self, DegreeOfSuccess, RollResult}}, spec::{Kingdom, skills::Skill}, turns::TurnState};

pub fn go_fishing(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext) -> (RollResult, TurnState, KingdomState) {
    let the_roll = kingdom.roll(Skill::Boating, context);
    let dc = DC(14); // TODO

    let degree = roll_result::rate_success(
        the_roll.natural,
        the_roll.total,
        dc,
    );

    let food_increase = match degree {
        DegreeOfSuccess::CriticalSuccess => context.d4,
        DegreeOfSuccess::Success         => 1,
        _                                => 0,
    };

    let unrest_increase = match degree {
        DegreeOfSuccess::CriticalFailure => context.d4,
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

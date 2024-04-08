use crate::{rolls::{bonus, roll_context::RollContext, roll_result::{self, DegreeOfSuccess, RollResult}}, spec::{skills::Skill, Kingdom}, state::{Commodity, KingdomState}, turns::TurnState};

pub fn purchase_commodities(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext, primary_want: Commodity, secondary_want: Commodity) -> (RollResult, TurnState, KingdomState) {
    let the_roll = kingdom.roll(state, bonus::KingdomAction::PurchaseCommodities, Skill::Industry, context);
    let dc = state.control_dc(kingdom);

    let degree = roll_result::rate_success(
        the_roll.natural,
        the_roll.total,
        dc,
    );

    let rp_cost = if primary_want == Commodity::Luxuries {8} else {4};

    let primary_increase = match degree {
        DegreeOfSuccess::CriticalSuccess => 4,
        DegreeOfSuccess::Success         => 2,
        DegreeOfSuccess::Failure         => 1,
        DegreeOfSuccess::CriticalFailure => 0,
    };

    let secondary_increase = match degree {
        DegreeOfSuccess::CriticalSuccess => 2,
        _                                => 0,
    };

    let next_turn_state = turn.clone();

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.resource_points -= rp_cost;
    next_kingdom_state.commodity_stores[primary_want] += primary_increase;
    next_kingdom_state.commodity_stores[secondary_want] += secondary_increase;

    let roll_result = RollResult {
        die_roll: the_roll,
        degree,
        dc,
    };

    (roll_result, next_turn_state, next_kingdom_state)
}

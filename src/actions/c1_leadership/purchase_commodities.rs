use crate::{state::{KingdomState, Commodity}, rolls::{roll_context::RollContext, roll_result::{DC, self, DegreeOfSuccess}, bonus::{AppliesUntil, Bonus, BonusType, AppliesTo}}, spec::{Kingdom, skills::Skill}, turns::TurnState};

pub fn purchase_commodities(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext, primary_want: Commodity, secondary_want: Commodity) -> (TurnState, KingdomState) {
    let mut next_turn_state = turn.clone();

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.resource_points -= 4;
    next_kingdom_state.commodity_stores[primary_want] += 4;
    next_kingdom_state.commodity_stores[secondary_want] += 2;

    (next_turn_state, next_kingdom_state)
}

use crate::{state::{KingdomState, Commodity}, rolls::{roll_context::RollContext, roll_result::{DC, self, DegreeOfSuccess}}, spec::{Kingdom, skills::Skill}, turns::TurnState};

pub fn trade_commodities(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext, commodity: Commodity, volume: i8) -> (TurnState, KingdomState) {
    let the_roll = kingdom.roll(Skill::Industry, context);
    let dc = DC(14); // TODO

    let degree = roll_result::rate_success(
        the_roll.natural,
        the_roll.total,
        dc,
    );

    let next_bonus_rp = match degree {
        DegreeOfSuccess::CriticalSuccess => 2 * volume,
        DegreeOfSuccess::Success         => volume,
        DegreeOfSuccess::Failure         => 1,
        DegreeOfSuccess::CriticalFailure => 0,
    };

    let next_turn_state = TurnState {
        traded_commodities: true,
        bonus_rp: turn.bonus_rp + next_bonus_rp,
        ..turn.clone()
    };

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.commodity_stores[commodity] -= volume; // TODO: check for <0
    if degree == DegreeOfSuccess::CriticalFailure && turn.traded_commodities {
        next_kingdom_state.unrest += 1;
    }

    (next_turn_state, next_kingdom_state)
}
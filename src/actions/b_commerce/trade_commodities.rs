use crate::{state::{KingdomState, Commodity}, rolls::{roll_context::RollContext, roll_result::{DC, self, DegreeOfSuccess}}, spec::{Kingdom, skills::Skill, attributes::Attribute}, turns::TurnState};

pub fn trade_commodities(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext, commodity: Commodity, volume: i8) -> (TurnState, KingdomState) {
    let next_turn_state = TurnState {
        bonus_rp: turn.bonus_rp + 4,
        ..turn.clone()
    };

    (next_turn_state, state.clone())
}

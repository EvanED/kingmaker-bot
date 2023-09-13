use crate::{state::KingdomState, rolls::{roll_context::RollContext, roll_result::{DC, self, DegreeOfSuccess}, bonus::{AppliesTo, AppliesUntil, BonusType, Bonus}}, spec::{Kingdom, skills::Skill}, turns::{TurnState, RandomEventSelectionMethod}};

pub fn prognosticate(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext) -> (TurnState, KingdomState) {
    let crit_success_bonus = Bonus {
        type_: BonusType::Circumstance,
        applies_to: AppliesTo::RandomEventResolutions,
        applies_until: AppliesUntil::StartOfTheNextTurn,  // FIXME: End of this turn vs start of next?
        modifier: 2,
        reason: "critical success on Prognostication".to_string(),
    };

    let next_kingdom_state = state.clone();
    
    let mut next_turn_state = turn.clone();
    next_turn_state.bonuses.push(crit_success_bonus);
    assert!(next_turn_state.random_event_selection_method.is_none());
    next_turn_state.random_event_selection_method = Some(RandomEventSelectionMethod::AdvantagePlayers);

    (next_turn_state, next_kingdom_state)
}

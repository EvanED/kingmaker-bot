use crate::{state::KingdomState, rolls::{roll_context::RollContext, roll_result::{DC, self, DegreeOfSuccess}, bonus::{AppliesTo, AppliesUntil, BonusType, Bonus}}, spec::{Kingdom, skills::Skill}, turns::{TurnState, RandomEventSelectionMethod}};

pub fn prognosticate(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext) -> (TurnState, KingdomState) {
    let the_roll = kingdom.roll(Skill::Industry, context);
    let dc = DC(14); // TODO

    let degree = roll_result::rate_success(
        the_roll.natural,
        the_roll.total,
        dc,
    );

    let modifier = match degree {
        DegreeOfSuccess::CriticalSuccess => 2,
        DegreeOfSuccess::Success         => 1,
        _                                => 0,
    };
    let crit_success_bonus = Bonus {
        type_: BonusType::Circumstance,
        applies_to: AppliesTo::RandomEventResolutions,
        applies_until: AppliesUntil::StartOfTheNextTurn,  // FIXME: End of this turn vs start of next?
        modifier,
        reason: "critical success on Prognostication".to_string(),
    };

    let event_selection = match degree {
        DegreeOfSuccess::CriticalSuccess => Some(RandomEventSelectionMethod::AdvantagePlayers),
        DegreeOfSuccess::CriticalFailure => Some(RandomEventSelectionMethod::AdvantageGM),
        _                                => None,
    };

    let next_kingdom_state = state.clone();
    
    let mut next_turn_state = turn.clone();
    if crit_success_bonus.modifier >= 1 {
        next_turn_state.bonuses.push(crit_success_bonus);
    }
    assert!(next_turn_state.random_event_selection_method.is_none());
    next_turn_state.random_event_selection_method = event_selection;

    (next_turn_state, next_kingdom_state)
}

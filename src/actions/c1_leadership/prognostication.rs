use crate::{rolls::{bonus::{self, AppliesTo, AppliesUntil, Bonus, BonusType}, roll_context::RollContext, roll_result::{self, DegreeOfSuccess, RollResult}}, spec::{skills::Skill, Kingdom}, state::KingdomState, turns::{RandomEventSelectionMethod, TurnState}};

pub fn prognosticate(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext) -> (RollResult, TurnState, KingdomState) {
    let the_roll = kingdom.roll(state, bonus::KingdomAction::Prognostication, Skill::Industry, context);
    let dc = state.control_dc(kingdom);

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
    let reason = match degree {
        DegreeOfSuccess::CriticalSuccess => "critical success on Prognostication",
        DegreeOfSuccess::Success         => "success on Prognostication",
        _                                => "YOU SHOULD NOT SEE THIS (Prognostication)",
    };
    let success_bonus = Bonus {
        type_: BonusType::Circumstance,
        applies_to: AppliesTo::RandomEventResolutions,
        applies_until: AppliesUntil::StartOfTheNextTurn,  // FIXME: End of this turn vs start of next?
        modifier,
        reason: reason.to_string(),
    };

    let event_selection = match degree {
        DegreeOfSuccess::CriticalSuccess => Some(RandomEventSelectionMethod::AdvantagePlayers),
        DegreeOfSuccess::CriticalFailure => Some(RandomEventSelectionMethod::AdvantageGM),
        _                                => None,
    };

    let next_kingdom_state = state.clone();
    
    let mut next_turn_state = turn.clone();
    if success_bonus.modifier >= 1 {
        next_turn_state.bonuses.push(success_bonus);
    }
    assert!(next_turn_state.random_event_selection_method.is_none());
    next_turn_state.random_event_selection_method = event_selection;

    let roll_result = RollResult {
        die_roll: the_roll,
        degree,
        dc,
    };

    (roll_result, next_turn_state, next_kingdom_state)
}

use crate::rolls::bonus::KingdomAction;
use crate::state::KingdomState;
use crate::rolls::{roll_context::RollContext, bonus::{Bonus, BonusType, AppliesTo, AppliesUntil}, roll_result::{self, DegreeOfSuccess, RollResult}};
use crate::spec::{Kingdom, skills::Skill, attributes::Attribute};
use crate::turns::TurnState;

pub fn improve_lifestyle(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext) -> (RollResult, TurnState, KingdomState) {
    let crit_success_bonus: Bonus = Bonus {
        type_: BonusType::Circumstance,
        applies_to: AppliesTo::Attribute(Attribute::Culture),
        applies_until: AppliesUntil::StartOfTheNextTurn,  // FIXME: End of this turn vs start of next?
        modifier: 2,
        reason: "critical success improving lifestyle".to_string(),
    };
    let success_bonus: Bonus = Bonus {
        type_: BonusType::Circumstance,
        applies_to: AppliesTo::Attribute(Attribute::Culture),
        applies_until: AppliesUntil::StartOfTheNextTurn,  // FIXME: End of this turn vs start of next?
        modifier: 1,
        reason: "success improving lifestyle".to_string(),
    };
    let failure_bonus: Bonus = Bonus {
        type_: BonusType::Circumstance,
        applies_to: AppliesTo::Attribute(Attribute::Culture),
        applies_until: AppliesUntil::StartOfTheNextTurn,  // FIXME: End of this turn vs start of next?
        modifier: 1,
        reason: "failure improving lifestyle (benefit)".to_string(),
    };
    let failure_penalty: Bonus = Bonus {
        type_: BonusType::Circumstance,
        applies_to: AppliesTo::Attribute(Attribute::Economy),
        applies_until: AppliesUntil::StartOfTheNextTurn,  // FIXME: End of this turn vs start of next?
        modifier: -1,
        reason: "failure improving lifestyle (drawback)".to_string(),
    };
    let crit_failure_penalty: Bonus = Bonus {
        type_: BonusType::Circumstance,
        applies_to: AppliesTo::Attribute(Attribute::Economy),
        applies_until: AppliesUntil::StartOfTheNextTurn,  // FIXME: End of this turn vs start of next?
        modifier: -1,
        reason: "critical failure improving lifestyle".to_string(),
    };
    

    let the_roll = kingdom.roll(state, KingdomAction::ImproveLifestyle, Skill::Politics, context);
    let dc = state.control_dc(kingdom);

    let degree = roll_result::rate_success(
        the_roll.natural,
        the_roll.total,
        dc,
    );

    let bonuses = match degree {
        DegreeOfSuccess::CriticalSuccess => vec![crit_success_bonus],
        DegreeOfSuccess::Success         => vec![success_bonus],
        DegreeOfSuccess::Failure         => vec![failure_bonus, failure_penalty],
        DegreeOfSuccess::CriticalFailure => vec![crit_failure_penalty],
    };

    let unrest_change = match degree {
        DegreeOfSuccess::CriticalFailure => 1,
        _                                => 0,
    };
    
    let mut next_turn_state = turn.clone();
    next_turn_state.bonuses.extend(bonuses);
    match degree {
        DegreeOfSuccess::CriticalFailure => next_turn_state.requirements.push("increase any Ruin".to_string()),
        _                                => (),
    };

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.unrest += unrest_change;  

    let roll_result = RollResult {
        die_roll: the_roll,
        degree,
        dc,
    };  
    
    (roll_result, next_turn_state, next_kingdom_state)
}


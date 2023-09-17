use crate::{state::KingdomState, rolls::{roll_context::RollContext, bonus::{Bonus, BonusType, AppliesTo, AppliesUntil}, roll_result::{DC, self, DegreeOfSuccess, RollResult, DieRoll, NaturalRoll, TotalRoll}}, spec::{Kingdom, skills::Skill, attributes::Attribute}, turns::TurnState};
use std::cmp;

pub fn decline_to_collect(_kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext) -> (RollResult, TurnState, KingdomState) {
    let natural = context.d20.roll();
    let unrest_change = if natural >= 11 {-1} else {0};
    let unrest = cmp::max(0, state.unrest + unrest_change);
    let new_kingdom_state = KingdomState { unrest, ..state.clone() }; // .clone() ??

    let roll_result = RollResult {
        die_roll: DieRoll {
            natural: NaturalRoll(natural),
            total:   TotalRoll(natural),
            description: "Collect Taxes: flat check to reduce unrest".to_string(),
        },
        degree: if natural >= 11 {DegreeOfSuccess::Success} else {DegreeOfSuccess::Failure},
        dc: DC(11),
    };

    (roll_result, turn.clone(), new_kingdom_state)
}

pub fn collect_taxes(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext) -> (RollResult, TurnState, KingdomState) {
    let crit_success_bonus: Bonus = Bonus {
        type_: BonusType::Circumstance,
        applies_to: AppliesTo::Attribute(Attribute::Economy),
        applies_until: AppliesUntil::StartOfTheNextTurn,  // FIXME: End of this turn vs start of next?
        modifier: 2,
        reason: "critical success collecting taxes".to_string(),
    };
    let success_bonus: Bonus = Bonus {
        type_: BonusType::Circumstance,
        applies_to: AppliesTo::Attribute(Attribute::Economy),
        applies_until: AppliesUntil::StartOfTheNextTurn,  // FIXME: End of this turn vs start of next?
        modifier: 1,
        reason: "success collecting taxes".to_string(),
    };
    let failure_bonus: Bonus = Bonus {
        type_: BonusType::Circumstance,
        applies_to: AppliesTo::Attribute(Attribute::Economy),
        applies_until: AppliesUntil::StartOfTheNextTurn,  // FIXME: End of this turn vs start of next?
        modifier: 1,
        reason: "failure collecting taxes".to_string(),
    };
    

    let the_roll = kingdom.roll(Skill::Trade, context);
    let dc = DC(14); // TODO

    let degree = roll_result::rate_success(
        the_roll.natural,
        the_roll.total,
        dc,
    );

    let the_bonus = match degree {
        DegreeOfSuccess::CriticalSuccess => Some(crit_success_bonus),
        DegreeOfSuccess::Success         => Some(success_bonus),
        DegreeOfSuccess::Failure         => Some(failure_bonus),
        DegreeOfSuccess::CriticalFailure => None,
    };

    let unrest_change = match degree {
        DegreeOfSuccess::CriticalSuccess => 0,
        DegreeOfSuccess::Success         => if turn.collected_taxes {1} else {0},
        DegreeOfSuccess::Failure         => if turn.collected_taxes {2} else {1},
        DegreeOfSuccess::CriticalFailure => 2,
    };
    

    let mut next_turn_state = turn.clone();
    match the_bonus {
        Some(the_bonus) => next_turn_state.bonuses.push(the_bonus),
        None => (),
    };
    match degree {
        DegreeOfSuccess::CriticalFailure => next_turn_state.requirements.push("increase any Ruin".to_string()),
        _ => (),
    };
    next_turn_state.collected_taxes = true;


    let mut next_kingdom_state = state.clone();
    next_kingdom_state.unrest += unrest_change;

    let roll_result = RollResult {
        die_roll: the_roll,
        degree,
        dc,
    };
    
    (roll_result, next_turn_state, next_kingdom_state)
}


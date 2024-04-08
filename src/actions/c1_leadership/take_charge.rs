use crate::{rolls::{bonus::{self, AppliesTo, AppliesUntil, Bonus, BonusType}, roll_context::RollContext, roll_result::{self, DegreeOfSuccess, RollResult}}, spec::{skills::Skill, Kingdom}, state::KingdomState, turns::TurnState};

pub fn take_charge(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext, skill: Skill) -> (RollResult, TurnState, KingdomState) {
    let crit_success_bonus = Bonus {
        type_: BonusType::Circumstance,
        applies_to: AppliesTo::Skill(skill),
        applies_until: AppliesUntil::NextApplicableRoll,  // FIXME: End of this turn vs start of next?
        modifier: 1,
        reason: "critical success on Take Charge".to_string(),
    };
    let crit_fail_penalty = Bonus {
        type_: BonusType::Circumstance,
        applies_to: AppliesTo::Skill(skill),
        applies_until: AppliesUntil::NextApplicableRoll,  // FIXME: End of this turn vs start of next?
        modifier: -1,
        reason: "critical failure on Take Charge".to_string(),
    };

    let the_roll = kingdom.roll(bonus::KingdomAction::TakeCharge, Skill::Industry, context);
    let dc = state.control_dc(kingdom);

    let degree = roll_result::rate_success(
        the_roll.natural,
        the_roll.total,
        dc,
    );

    let gained_rp = match degree {
        DegreeOfSuccess::CriticalSuccess => 1,
        DegreeOfSuccess::Success         => 1,
        DegreeOfSuccess::Failure         => 0,
        DegreeOfSuccess::CriticalFailure => 0,
    };

    let mut next_state = state.clone();
    next_state.resource_points += gained_rp;

    let mut next_turn_state = turn.clone();
    match degree {
        DegreeOfSuccess::CriticalSuccess => next_turn_state.bonuses.push(crit_success_bonus),
        DegreeOfSuccess::Success         => (),
        DegreeOfSuccess::Failure         => (),
        DegreeOfSuccess::CriticalFailure => next_turn_state.bonuses.push(crit_fail_penalty),
    }

    let roll_result = RollResult {
        die_roll: the_roll,
        degree,
        dc,
    };

    (roll_result, next_turn_state, next_state)
}

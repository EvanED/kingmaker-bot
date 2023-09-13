use crate::{state::KingdomState, rolls::{roll_context::RollContext, roll_result::{DC, self, DegreeOfSuccess}, bonus::{AppliesUntil, Bonus, BonusType, AppliesTo}}, spec::{Kingdom, skills::Skill}, turns::TurnState};

pub fn take_charge(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext, skill: Skill) -> (TurnState, KingdomState) {
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

    let the_roll = kingdom.roll(Skill::Industry, context);
    let dc = DC(14); // TODO

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

    (next_turn_state, next_state)
}

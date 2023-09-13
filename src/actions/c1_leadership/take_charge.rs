use crate::{state::{KingdomState, Commodity}, rolls::{roll_context::RollContext, roll_result::{DC, self, DegreeOfSuccess}, bonus::{AppliesUntil, Bonus, BonusType, AppliesTo}}, spec::{Kingdom, skills::Skill}, turns::TurnState};

pub fn take_charge(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext, skill: Skill) -> (TurnState, KingdomState) {
    let crit_success_bonus = Bonus {
        type_: BonusType::Circumstance,
        applies_to: AppliesTo::Skill(skill),
        applies_until: AppliesUntil::NextApplicableRoll,  // FIXME: End of this turn vs start of next?
        modifier: 1,
        reason: "critical success on Take Charge".to_string(),
    };

    let mut next_state = state.clone();
    next_state.resource_points += 1;

    let mut next_turn_state = turn.clone();
    next_turn_state.bonuses.push(crit_success_bonus);

    (next_turn_state, next_state)
}

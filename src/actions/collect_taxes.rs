use crate::{state::KingdomState, rolls::{roll_context::RollContext, bonus::{Bonus, BonusType, AppliesTo, AppliesUntil}, roll_result::{DC, self, DegreeOfSuccess}}, spec::{Kingdom, skills::Skill, attributes::Attribute}, turns::TurnState};

pub fn collect_taxes(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext) -> (TurnState, KingdomState) {
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
        DegreeOfSuccess::CriticalSuccess => crit_success_bonus,
        DegreeOfSuccess::Success         => success_bonus,
        DegreeOfSuccess::Failure         => failure_bonus,
        _                                => panic!("Blahblah")
    };

    let unrest_change = match degree {
        DegreeOfSuccess::CriticalSuccess => 0,
        DegreeOfSuccess::Success         => if turn.collected_taxes {1} else {0},
        DegreeOfSuccess::Failure         => if turn.collected_taxes {2} else {1},
        _                                => panic!("Blahblah")
    };
    

    let mut next_turn_state = turn.clone();
    next_turn_state.bonuses.push(the_bonus);
    next_turn_state.collected_taxes = true;

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.unrest += unrest_change;    
    
    (next_turn_state, next_kingdom_state)
}


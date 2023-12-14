use crate::{state::KingdomState, rolls::{roll_context::RollContext, roll_result::{self, DegreeOfSuccess, RollResult}, bonus::Bonus}, spec::{Kingdom, skills::Skill, attributes::Attribute}, turns::TurnState};

pub fn celebrate_holiday(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext) -> (RollResult, TurnState, KingdomState) {
    let the_roll = kingdom.roll(Skill::Industry, context);
    let d4 = context.d4.roll();
    let dc = state.control_dc(kingdom);

    let degree = roll_result::rate_success(
        the_roll.natural,
        the_roll.total,
        dc,
    );

    let loyalty_bonus = match degree {
        DegreeOfSuccess::CriticalSuccess =>  2,
        DegreeOfSuccess::Success         =>  1,
        DegreeOfSuccess::Failure         =>  0,
        DegreeOfSuccess::CriticalFailure => -1,
    };

    let roll_a_recource_dice_spend_rp_equal_to_the_result = match degree {
        DegreeOfSuccess::CriticalSuccess => 0,
        _                                => d4,
    };

    let mut next_turn_state = turn.clone();
    if loyalty_bonus != 0 {
        let bonus = Bonus {
            type_: crate::rolls::bonus::BonusType::Circumstance,
            applies_to: crate::rolls::bonus::AppliesTo::Attribute(Attribute::Loyalty),
            applies_until: crate::rolls::bonus::AppliesUntil::EndOfTheNextTurn,
            modifier: loyalty_bonus,
            reason: format!("{} celebrating holiday", degree.lowercase_description()),
        };
        next_turn_state.bonuses.push(bonus);
    }
    if degree == DegreeOfSuccess::CriticalFailure {
        next_turn_state.requirements.push(
            "there is a penalty of 4 resource dice next turn".to_string()
        );
    }

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.resource_points -= roll_a_recource_dice_spend_rp_equal_to_the_result;

    let roll_result = RollResult {
        die_roll: the_roll,
        degree,
        dc,
    };

    (roll_result, next_turn_state, next_kingdom_state)
}

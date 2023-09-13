use crate::{state::KingdomState, rolls::{roll_context::RollContext, roll_result::{DC, self, DegreeOfSuccess}}, spec::{Kingdom, skills::Skill}, turns::TurnState};

pub fn celebrate_holiday(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext) -> (TurnState, KingdomState) {
    let the_roll = kingdom.roll(Skill::Industry, context);
    let dc = DC(14); // TODO

    let degree = roll_result::rate_success(
        the_roll.natural,
        the_roll.total,
        dc,
    );
    
    let gain_1_fame_or_infamy_point_immediately = match degree {
        DegreeOfSuccess::CriticalSuccess => 1,
        DegreeOfSuccess::Success         => 1,
        DegreeOfSuccess::Failure         => 0,
        DegreeOfSuccess::CriticalFailure => -1,
    };

    let at_the_start_of_your_next_kingdom_turn_gain_1_additional_fame_or_infamy_point = match degree {
        DegreeOfSuccess::CriticalSuccess => 1,
        _                                => 0,
    };

    let roll_2_recource_dice_gain_rp_equal_to_the_result = match degree {
        DegreeOfSuccess::CriticalSuccess => context.d4 + context.d4,
        _                                => 0,
    };

    let mut next_turn_state = turn.clone();
    next_turn_state.additional_fame_points_scheduled += at_the_start_of_your_next_kingdom_turn_gain_1_additional_fame_or_infamy_point;

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.fame_points += gain_1_fame_or_infamy_point_immediately;
    next_kingdom_state.resource_points += roll_2_recource_dice_gain_rp_equal_to_the_result;

    if next_kingdom_state.fame_points < 0 {
        assert_eq!(degree, DegreeOfSuccess::CriticalFailure);
        next_kingdom_state.fame_points = 0;
        next_kingdom_state.unrest += context.d4;
    }

    (next_turn_state, next_kingdom_state)
}

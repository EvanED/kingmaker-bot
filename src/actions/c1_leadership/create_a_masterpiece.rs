use crate::{rolls::{bonus, roll_context::RollContext, roll_result::{self, DegreeOfSuccess, RollResult}}, spec::{skills::Skill, Kingdom}, state::KingdomState, turns::TurnState};

pub fn create_a_masterpiece(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext) -> (RollResult, TurnState, KingdomState) {
    let the_roll = kingdom.roll(state, bonus::KingdomAction::CreateAMasterpiece, Skill::Arts, context);
    let d4_1 = context.d4.roll();
    let d4_2 = context.d4.roll();
    let dc = state.control_dc(kingdom);

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
        DegreeOfSuccess::CriticalSuccess => d4_1 + d4_2,
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
        next_kingdom_state.unrest += d4_1;
    }

    let roll_result = RollResult {
        die_roll: the_roll,
        degree,
        dc,
    };

    (roll_result, next_turn_state, next_kingdom_state)
}

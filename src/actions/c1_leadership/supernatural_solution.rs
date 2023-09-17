use crate::{state::KingdomState, rolls::{roll_context::RollContext, roll_result::{DC, self, DegreeOfSuccess, RollResult}}, spec::{Kingdom, skills::Skill}, turns::TurnState};

pub fn supernatural_solution(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext) -> (RollResult, TurnState, KingdomState) {
    let the_roll = kingdom.roll(Skill::Industry, context);
    let d4 = context.d4.roll();
    let d6_1 = context.d6.roll();
    let d6_2 = context.d6.roll();
    let dc = DC(14); // TODO

    let degree = roll_result::rate_success(
        the_roll.natural,
        the_roll.total,
        dc,
    );

    let research_successful = match degree {
        DegreeOfSuccess::CriticalSuccess => true,
        DegreeOfSuccess::Success         => true,
        _                                => false,
    };

    let research_cost = match degree {
        DegreeOfSuccess::CriticalSuccess => 0,
        DegreeOfSuccess::Success         => d4,
        DegreeOfSuccess::Failure         => d6_1 + d6_2,
        DegreeOfSuccess::CriticalFailure => d6_1 + d6_2,
    };

    let mut next_turn_state = turn.clone();
    next_turn_state.supernatural_solution_available |= research_successful;

    if degree == DegreeOfSuccess::CriticalFailure {
        next_turn_state.supernatural_solution_blocked_for_x_turns = Some(2);
    }

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.resource_points -= research_cost;

    let roll_result = RollResult {
        die_roll: the_roll,
        degree,
        dc,
    };

    (roll_result, next_turn_state, next_kingdom_state)
}

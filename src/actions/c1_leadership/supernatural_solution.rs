use crate::{state::KingdomState, rolls::{roll_context::RollContext, roll_result::{DC, self, DegreeOfSuccess}}, spec::{Kingdom, skills::Skill}, turns::TurnState};

pub fn supernatural_solution(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext) -> (TurnState, KingdomState) {
    let the_roll = kingdom.roll(Skill::Industry, context);
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
        DegreeOfSuccess::Success         => context.d4,
        DegreeOfSuccess::Failure         => context.d6 + context.d6,
        DegreeOfSuccess::CriticalFailure => context.d6 + context.d6,
    };

    let mut next_turn_state = turn.clone();
    next_turn_state.supernatural_solution_available |= research_successful;

    if degree == DegreeOfSuccess::CriticalFailure {
        next_turn_state.supernatural_solution_blocked_for_x_turns = Some(2);
    }

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.resource_points -= research_cost;

    (next_turn_state, next_kingdom_state)
}

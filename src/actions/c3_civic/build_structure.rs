use enum_map::Enum;

use crate::{state::{KingdomState, Commodity}, rolls::{roll_context::RollContext, roll_result::{DC, self, DegreeOfSuccess}}, spec::{Kingdom, skills::Skill}, turns::TurnState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
pub enum Structure {
    Shrine,
}

pub fn build_structure(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext, structure: Structure) -> (TurnState, KingdomState) {
    let skill = Skill::Folklore;
    let dc = 15;
    let rp_cost = 8;
    let stone_cost = 3; // actually others, for Shrine

    build_structure_from_stats(kingdom, turn, state, context, structure, skill, dc, rp_cost, stone_cost)
}

pub fn build_structure_from_stats(
    kingdom: &Kingdom,
    turn: &TurnState,
    state: &KingdomState,
    context: &RollContext,
    // structure stats:
    structure: Structure,
    skill: Skill,
    dc: i8,
    rp_cost: i8,
    stone_cost: i8,
) -> (TurnState, KingdomState)
{
    let the_roll = kingdom.roll(skill, context);
    let dc = DC(dc); // TODO

    let degree = roll_result::rate_success(
        the_roll.natural,
        the_roll.total,
        dc,
    );

    let grid_msg = "mark the urban grid with the new stucture".to_string();
    let bonuses_msg = "adjust kingdom item bonuses accordingly".to_string();
    let new_requirements = match degree {
        DegreeOfSuccess::CriticalSuccess => vec![grid_msg, bonuses_msg],
        DegreeOfSuccess::Success         => vec![grid_msg, bonuses_msg],
        _                                => vec![],
    };

    let stone_cost = match degree {
        DegreeOfSuccess::CriticalSuccess => stone_cost - stone_cost / 2,  // Get half back
        _                                => stone_cost,
    };

    let mut next_turn_state = turn.clone();
    next_turn_state.requirements.extend(new_requirements);
    if degree == DegreeOfSuccess::Failure {
        assert_eq!(None, next_turn_state.can_build_this_structure_for_no_resource_cost);
        next_turn_state.can_build_this_structure_for_no_resource_cost = Some(structure);
    }
    if degree == DegreeOfSuccess::CriticalFailure {
        next_turn_state.requirements.push("fill the lot(s) in the Urban Grid with rubble".to_string());
    }

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.resource_points -= rp_cost;
    next_kingdom_state.commodity_stores[Commodity::Stone] -= stone_cost;

    (next_turn_state, next_kingdom_state)
}

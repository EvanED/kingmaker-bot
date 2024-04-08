use poise::ChoiceParameter;

use crate::{discord::commands::action::next_turn, rolls::{bonus::{self, AppliesTo, AppliesUntil, Bonus, BonusType}, roll_context::RollContext, roll_result::{self, DegreeOfSuccess, RollResult}}, spec::{attributes::Attribute, skills::Skill, Kingdom}, state::{HexCoordinate, KingdomState}, turns::TurnState};

#[derive(Debug, Clone, Copy, ChoiceParameter)]
pub enum ClaimHexSkill {
    Exploration,
    Intrigue,
    Wilderness,
    Magic,
}

// TODO: Require Skill as one of prereqs
pub fn claim_hex(
    kingdom: &Kingdom,
    turn: &TurnState,
    state: &KingdomState,
    context: &RollContext,
    skill: ClaimHexSkill,
    x: i8,
    y: i8,
) -> (RollResult, TurnState, KingdomState) {

    let skill = match skill {
        ClaimHexSkill::Exploration => Skill::Exploration,
        ClaimHexSkill::Intrigue    => Skill::Intrigue,
        ClaimHexSkill::Magic       => Skill::Magic,
        ClaimHexSkill::Wilderness  => Skill::Wilderness,
    };

    let the_roll = kingdom.roll(bonus::KingdomAction::ClaimHex, skill, context);
    let dc = state.control_dc(kingdom);

    let degree = roll_result::rate_success(
        the_roll.natural,
        the_roll.total,
        dc,
    );

    let new_msg = "mark the hex as claimed".to_string();
    let xp_msg = "gain XP from the claimed hex".to_string();
    let next = "you may take another region activity".to_string();
    let new_requirements = match degree {
        DegreeOfSuccess::CriticalSuccess => vec![new_msg, xp_msg, next],
        DegreeOfSuccess::Success         => vec![new_msg, xp_msg],
        _                                => vec![],
    };

    let crit_failure_penalty: Bonus = Bonus {
        type_: BonusType::Circumstance,
        applies_to: AppliesTo::Attribute(Attribute::Stability),
        applies_until: AppliesUntil::EndOfTheNextTurn,  // FIXME: End of this turn vs start of next?
        modifier: -1,
        reason: "critical failure in Claim Hex".to_string(),
    };


    let mut next_turn_state = turn.clone();
    next_turn_state.requirements.extend(new_requirements);
    if degree == DegreeOfSuccess::CriticalFailure {
        next_turn_state.bonuses.push(crit_failure_penalty);
    }

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.resource_points -= 1;
    if degree.passed() {
        next_kingdom_state.claimed_hexes.push(
            HexCoordinate { x, y }
        )
    }

    let roll_result = RollResult {
        die_roll: the_roll,
        degree,
        dc,
    };

    (roll_result, next_turn_state, next_kingdom_state)
}


// TODO: Is Claim Hex DC really independent of terrain type
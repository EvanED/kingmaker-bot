use cucumber::then;
use assert2::assert;
use kingdom::{rolls::{bonus::{BonusType, AppliesTo, AppliesUntil}, roll_result::DegreeOfSuccess}, spec::{attributes::Attribute, skills::Skill}};
use crate::context::TestContext;

#[then(expr = "I get a result of {int} \\(natural {int}\\)")]
fn check_roll_total(world: &mut TestContext, total_expected: i32, natural_expected: i32) {
    let total_actual: i32   = world.die_roll.as_ref().unwrap().total.0.into();
    let natural_actual: i32 = world.die_roll.as_ref().unwrap().natural.0.into();

    assert_eq!(total_expected, total_actual);
    assert_eq!(natural_expected, natural_actual);
}

#[then(expr = "the roll description is {string}")]
fn check_roll_description(world: &mut TestContext, description_expected: String) {
    assert!(description_expected == world.die_roll.as_ref().unwrap().description);
}

#[then(expr = "there are no remaining bonuses")]
fn check_no_remaining_bonuses(world: &mut TestContext) {
    assert!(0 == world.remaining_bonuses.len());
}

#[then(expr = "there this is 1 remaining bonus")]
fn check_number_remaining_bonuses(world: &mut TestContext) {
    assert!(1 == world.remaining_bonuses.len());
}

#[then(expr = "{string} is in remaining bonuses")]
fn check_containing_bonus(world: &mut TestContext, desc: String) {
    for bonus in world.remaining_bonuses.iter() {
        if bonus.reason == desc {
            return;
        }
    }
    assert!(false, "could not find required bonus")
}

#[then("there is no bonus")]
fn check_there_is_no_bonus(world: &mut TestContext) {
    assert!(0 == world.next_turn_state.bonuses.len());
}

#[then("there is one bonus")]
fn check_there_is_one_bonus(world: &mut TestContext) {
    assert!(1 == world.next_turn_state.bonuses.len());
}

#[then("there are two bonuses")]
fn check_there_are_two_bonuses(world: &mut TestContext) {
    assert!(1 == world.next_turn_state.bonuses.len());
}

#[then(expr = "there is a +{int} circumstance bonus to Economy until the end of the turn, because {string}")]
fn check_there_is_plus2_circumstance_bonus_to_economy_until_end_of_the_turn(world: &mut TestContext, modifier: i32, reason: String) {
    assert!(1 == world.next_turn_state.bonuses.len());

    let bonus = &world.next_turn_state.bonuses[0];
    assert!(bonus.type_ == BonusType::Circumstance);
    assert!(bonus.applies_to == AppliesTo::Attribute(Attribute::Economy));
    assert!(bonus.applies_until == AppliesUntil::StartOfTheNextTurn);
    assert!(bonus.modifier as i32 == modifier);
    assert!(bonus.reason == reason);
}

#[then(expr = "there is a {int} circumstance penalty to Economy until the end of the turn, because {string}")]
fn check_there_is_circumstance_penalty_to_economy_until_end_of_the_turn(world: &mut TestContext, modifier: i32, reason: String) {
    assert!(1 == world.next_turn_state.bonuses.len());

    let bonus = &world.next_turn_state.bonuses[0];
    assert!(bonus.type_ == BonusType::Circumstance);
    assert!(bonus.applies_to == AppliesTo::Attribute(Attribute::Economy));
    assert!(bonus.applies_until == AppliesUntil::StartOfTheNextTurn);
    assert!(bonus.modifier as i32 == modifier);
    assert!(bonus.reason == reason);
}

#[then(expr = "there is a +{int} circumstance bonus to Culture until the end of the turn, because {string}")]
fn check_there_is_plus2_circumstance_bonus_to_culture_until_end_of_the_turn(world: &mut TestContext, modifier: i32, reason: String) {
    assert!(1 == world.next_turn_state.bonuses.len());

    let bonus = &world.next_turn_state.bonuses[0];
    assert!(bonus.type_ == BonusType::Circumstance);
    assert!(bonus.applies_to == AppliesTo::Attribute(Attribute::Culture));
    assert!(bonus.applies_until == AppliesUntil::StartOfTheNextTurn);
    assert!(bonus.modifier as i32 == modifier);
    assert!(bonus.reason == reason);
}

#[then(expr = "there exists a +{int} circumstance bonus to Culture until the end of the turn, because {string}")]
fn check_there_exists_bonus(world: &mut TestContext, modifier: i32, reason: String) {
    let found = world.next_turn_state.bonuses.iter().filter(
        |bonus| {
            (bonus.type_ == BonusType::Circumstance)
            && (bonus.applies_to == AppliesTo::Attribute(Attribute::Culture))
            && (bonus.applies_until == AppliesUntil::StartOfTheNextTurn)
            && (bonus.modifier as i32 == modifier)
            && (bonus.reason == reason)
        }
    ).count();

    assert!(1 == found, "Bonuses: {:?}", world.next_turn_state.bonuses);
}

#[then(expr = "there exists a {int} circumstance penalty to Economy until the end of the turn, because {string}")]
fn check_there_exists_bonus2(world: &mut TestContext, modifier: i32, reason: String) {
    let found = world.next_turn_state.bonuses.iter().filter(
        |bonus| {
            (bonus.type_ == BonusType::Circumstance)
            && (bonus.applies_to == AppliesTo::Attribute(Attribute::Economy))
            && (bonus.applies_until == AppliesUntil::StartOfTheNextTurn)
            && (bonus.modifier as i32 == modifier)
            && (bonus.reason == reason)
        }
    ).count();

    assert!(1 == found, "Bonuses: {:?}", world.next_turn_state.bonuses);
}


#[then(expr = "there is a +{int} circumstance bonus to Arts on the next check, because {string}")]
fn there_is_bonus_arts(world: &mut TestContext, modifier: i32, reason: String) {
    assert!(1 == world.next_turn_state.bonuses.len());

    let bonus = &world.next_turn_state.bonuses[0];
    assert!(bonus.type_ == BonusType::Circumstance);
    assert!(bonus.applies_to == AppliesTo::Skill(Skill::Arts));
    assert!(bonus.applies_until == AppliesUntil::NextApplicableRoll);
    assert!(bonus.modifier as i32 == modifier);
    assert!(bonus.reason == reason);
}

#[then(expr = "there is a -1 circumstance penalty to Arts on the next check, because {string}")]
fn there_is_penalty_arts(world: &mut TestContext, reason: String) {
    assert!(1 == world.next_turn_state.bonuses.len());

    let bonus = &world.next_turn_state.bonuses[0];
    assert!(bonus.type_ == BonusType::Circumstance);
    assert!(bonus.applies_to == AppliesTo::Skill(Skill::Arts));
    assert!(bonus.applies_until == AppliesUntil::NextApplicableRoll);
    assert!(bonus.modifier == -1);
    assert!(bonus.reason == reason);
}

#[then(expr = "there is a +{int} circumstance bonus to The Event until the end of the turn, because {string}")]
fn there_is_event_bonus(world: &mut TestContext, modifier: i32, reason: String) {
    assert!(1 == world.next_turn_state.bonuses.len());

    let bonus = &world.next_turn_state.bonuses[0];
    assert!(bonus.type_ == BonusType::Circumstance);
    assert!(bonus.applies_to == AppliesTo::RandomEventResolutions);
    assert!(bonus.applies_until == AppliesUntil::StartOfTheNextTurn); // TODO: End of turn, technically
    assert!(bonus.modifier as i32 == modifier);
    assert!(bonus.reason == reason);
}

#[then(expr = "there is a -1 circumstance penalty to Stability until the end of the next turn, because {string}")]
fn then_there_is_stability_penalty(world: &mut TestContext, reason: String) {
    assert!(1 == world.next_turn_state.bonuses.len());

    let bonus = &world.next_turn_state.bonuses[0];
    assert!(bonus.type_ == BonusType::Circumstance);
    assert!(bonus.applies_to == AppliesTo::Attribute(Attribute::Stability));
    assert!(bonus.applies_until == AppliesUntil::EndOfTheNextTurn);
    assert!(bonus.modifier == -1);
    assert!(bonus.reason == reason);
}

#[then("the roll result was a success")]
fn then_the_die_roll_was_a_success(world: &mut TestContext) {
    assert!(world.roll_result.is_some());
    assert!(world.roll_result.as_ref().unwrap().degree == DegreeOfSuccess::Success);
}

#[then("the roll result was a critical success")]
fn then_the_die_roll_was_a_critical_success(world: &mut TestContext) {
    assert!(world.roll_result.is_some());
    assert!(world.roll_result.as_ref().unwrap().degree == DegreeOfSuccess::CriticalSuccess);
}

#[then(expr = "the roll result was a natural {int}")]
fn then_the_roll_result_was_a_natural(world: &mut TestContext, expected_result: i32) {
    assert!(world.roll_result.as_ref().unwrap().die_roll.natural.0 as i32 == expected_result);
}

#[then(expr = "the roll result was a total {int}")]
fn then_the_roll_result_was_a_total(world: &mut TestContext, expected_result: i32) {
    assert!(world.roll_result.as_ref().unwrap().die_roll.total.0 as i32 == expected_result);
}

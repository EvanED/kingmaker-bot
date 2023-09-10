use cucumber::then;
use assert2::assert;
use crate::context::TestContext;

#[then(expr = "I get a result of {int} \\(natural {int}\\)")]
fn check_roll_total(world: &mut TestContext, total_expected: i32, natural_expected: i32) {
    let total_actual: i32   = world.roll_result.as_ref().unwrap().total.into();
    let natural_actual: i32 = world.roll_result.as_ref().unwrap().natural.into();

    assert_eq!(total_expected, total_actual);
    assert_eq!(natural_expected, natural_actual);
}

#[then(expr = "the roll description is {string}")]
fn check_roll_description(world: &mut TestContext, description_expected: String) {
    assert!(description_expected == world.roll_result.as_ref().unwrap().description);
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

use cucumber::then;
use crate::context::AnimalWorld;

#[then(expr = "I get a result of {int} \\(natural {int}\\)")]
fn check_roll_total(world: &mut AnimalWorld, total_expected: i32, natural_expected: i32) {
    let total_actual: i32   = world.roll_result.as_ref().unwrap().total.into();
    let natural_actual: i32 = world.roll_result.as_ref().unwrap().natural.into();

    assert_eq!(total_expected, total_actual);
    assert_eq!(natural_expected, natural_actual);
}

use cucumber::then;
use crate::context::AnimalWorld;

#[then(expr = "I get a result of {int}")]
fn check_roll_total(world: &mut AnimalWorld, _total: i32) {
    assert_eq!(14, world.roll_result.as_ref().unwrap().0);
}

#[then("the cat is not hungry")]
fn cat_is_fed(world: &mut AnimalWorld) {
    assert!(!world.cat.hungry);
}

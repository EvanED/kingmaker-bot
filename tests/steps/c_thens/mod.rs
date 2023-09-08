use cucumber::then;
use crate::myworld::AnimalWorld;

#[then("the cat is not hungry")]
fn cat_is_fed(world: &mut AnimalWorld) {
    assert!(!world.cat.hungry);
}
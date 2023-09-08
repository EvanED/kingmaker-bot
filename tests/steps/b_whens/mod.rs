use cucumber::when;
use crate::myworld::AnimalWorld;

#[when("I feed the cat")]
fn feed_cat(world: &mut AnimalWorld) {
    world.cat.feed();
}

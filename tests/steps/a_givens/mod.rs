use cucumber::given;
use crate::context::AnimalWorld;

#[given("a hungry cat")]
fn hungry_cat(world: &mut AnimalWorld) {
    world.cat.hungry = true;
}

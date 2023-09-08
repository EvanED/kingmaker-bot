use cucumber::when;
use kingdom::spec::Skill;
use crate::context::AnimalWorld;

#[when("I roll Arts")]
fn roll_arts(world: &mut AnimalWorld) {
    let roll_result = world.kingdom.as_ref().unwrap()
        .roll(Skill::Arts, world.roll);
    world.roll_result = Some(roll_result);
}

#[when("I feed the cat")]
fn feed_cat(world: &mut AnimalWorld) {
    world.cat.feed();
}

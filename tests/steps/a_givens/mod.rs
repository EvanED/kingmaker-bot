use cucumber::given;
use crate::context::AnimalWorld;
use kingdom::spec::{self, enum_map, TrainingLevel, Attribute, Skill};

#[given("the kingdom Aryc at level 1")]
fn create_aryc(world: &mut AnimalWorld) {
    use Attribute::*;
    use Skill::*;
    use TrainingLevel::*;
    let aryc = spec::Kingdom {
        level: 1,
        attributes: enum_map! {
            Culture   => 0,
            Economy   => 4,
            Loyalty   => 2,
            Stability => 2,
        },
        invested: enum_map! {
            Culture   => true,
            Economy   => true,
            Loyalty   => true,
            Stability => true,
        },
        skills: enum_map! {
            Agriculture => Untrained,
            Arts        => Trained,
            Boating     => Trained,
            Defense     => Trained,
            Engineering => Trained,
            Exploration => Untrained,
            Folklore    => Untrained,
            Industry    => Trained,
            Intrigue    => Untrained,
            Magic       => Trained,
            Politics    => Trained,
            Scholarship => Untrained,
            Statecraft  => Untrained,
            Trade       => Trained,
            Warfare     => Trained,
            Wilderness  => Untrained,
        },
    };
    world.kingdom = Some(aryc);
}

#[given("a die roll of 10")]
fn set_roll(world: &mut AnimalWorld) {
    world.roll = 10;
}

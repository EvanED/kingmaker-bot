use cucumber::given;
use crate::context::TestContext;
use kingdom::spec::{self, enum_map};
use kingdom::spec::attributes::Attribute;
use kingdom::spec::skills::{TrainingLevel, Skill};

#[given("the kingdom Aryc at level 1")]
fn create_aryc(world: &mut TestContext) {
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

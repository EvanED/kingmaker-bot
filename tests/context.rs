use kingdom::{Cat, spec::Kingdom};
use kingdom::roll_result::RollResult;
use cucumber::World;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario. 
#[derive(Debug, Default, World)]
pub struct AnimalWorld {
    pub cat: Cat,
    pub kingdom: Option<Kingdom>,
    pub roll: i8,
    pub roll_result: Option<RollResult>,
}

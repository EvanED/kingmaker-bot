use kingdom::roll_context::RollContext;
use kingdom::spec::Kingdom;
use kingdom::roll_result::RollResult;
use cucumber::World;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario. 
#[derive(Debug, Default, World)]
pub struct TestContext {
    pub kingdom:      Option<Kingdom>,
    pub roll_context: Option<RollContext>,
    pub roll_result:  Option<RollResult>,
}

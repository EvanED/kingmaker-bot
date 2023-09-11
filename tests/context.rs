use kingdom::rolls::bonus::Bonus;
use kingdom::rolls::roll_context::RollContext;
use kingdom::spec::Kingdom;
use kingdom::rolls::roll_result::RollResult;
use cucumber::World;
use kingdom::state::KingdomState;
use kingdom::turns::TurnState;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario. 
#[derive(Debug, Default, World)]
pub struct TestContext {
    pub kingdom:           Option<Kingdom>,
    pub roll_context:      Option<RollContext>,
    pub roll_result:       Option<RollResult>,
    pub remaining_bonuses: Vec<Bonus>,
    pub turn_state:        TurnState,
    pub next_turn_state:   TurnState,
    pub kingdom_state:     KingdomState,
}

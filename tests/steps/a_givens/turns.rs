use cucumber::given;
use kingdom::turns::TurnState;
use crate::context::TestContext;


#[given("the kingdom did not collect taxes the previous turn")]
fn given_the_kingdom_did_not_collect_taxes(world: &mut TestContext)
{
    world.turn_state = TurnState {
        collected_taxes: false,
        bonuses: Vec::new(),
        requirements: Vec::new(),
    };
}

#[given("the kingdom did collect taxes the previous turn")]
fn given_the_kingdom_did_collect_taxes(world: &mut TestContext)
{
    world.turn_state = TurnState {
        collected_taxes: true,
        bonuses: Vec::new(),
        requirements: Vec::new(),
    };
}

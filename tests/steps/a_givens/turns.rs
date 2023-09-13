use cucumber::given;
use crate::context::TestContext;


#[given("the kingdom did not collect taxes the previous turn")]
fn given_the_kingdom_did_not_collect_taxes(world: &mut TestContext)
{
    world.turn_state.collected_taxes = false;
}

#[given("the kingdom did collect taxes the previous turn")]
fn given_the_kingdom_did_collect_taxes(world: &mut TestContext)
{
    world.turn_state.collected_taxes = true;
}

#[given("the kingdom traded commodities the previous turn")]
fn given_the_kingdom_traded_commodities(world: &mut TestContext)
{
    world.turn_state.traded_commodities = true;
}
#[given("the kingdom is not scheduled to gain a Fame point at the start of next turn")]
fn given_the_kingdom_is_not_scheduled_to_gain_a_fame_point_at_the_start_of_next_turn(world: &mut TestContext) {
    world.turn_state.additional_fame_points_scheduled = 0;
}

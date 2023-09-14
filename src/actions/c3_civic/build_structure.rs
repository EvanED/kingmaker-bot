use enum_map::Enum;

use crate::{state::{KingdomState, Commodity}, rolls::{roll_context::RollContext, roll_result::{DC, self, DegreeOfSuccess}}, spec::{Kingdom, skills::Skill}, turns::TurnState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
pub enum Structure {
    // CAUTION: This order MUST be kept in sync with STRUCTURE_STATS below
    
    // Level 0
    Tenement,
    Houses,
    Orphange,

    // Level 1
    Brewery,
    Cemetery,
    GeneralStore,
    Granary,
    Herbalist,
    Inn,
    Shrine,
    TavernDive,
    WallWooden,

    // Level 2
    Bridge,
    Dump,
    Jail,
    Library,
    Mill,
    TownHall,

    // Level 3
    AlchemyLab,
    Barracks,
    Keep,
    FestivalHall,
    Lumberyard,
    Monument,
    Park,
    Pier,
    Smithy,
    Stable,
    Stockyard,
    Stonemason,
    Tannery,
    TavernPopular,
    TradeShop,
    Watchtower,
}

type StructureStatsTableRow = (Skill, i8, i8, i8, i8, i8, i8, i8, i8);

const STRUCTURE_STATS: &'static [StructureStatsTableRow] = &[
    // CAUTION: This order must be kept in sync with the definition of Structure above
    //
    //                               Housing
    //                      Skill             DC  RP   FD  LU  LX  OR  ST  More
    /* Tenement      */ (Skill::Industry    , 14,   1,  0,  1,  0,  0,  0,  0),
    /* Houses        */ (Skill::Industry    , 15,   3,  0,  1,  0,  0,  0,  0),
    /* Orphange      */ (Skill::Industry    , 16,   6,  0,  2,  0,  0,  0,  0),

    //                              Level 1
    //                      Skill             DC  RP   FD  LU  LX  OR  ST  More
    /* Brewery       */ (Skill::Agriculture , 15,   6,  0,  2,  0,  0,  0,  0),
    /* Cemetery      */ (Skill::Folklore    , 15,   4,  0,  0,  0,  0,  1,  0),
    /* GeneralStore  */ (Skill::Trade       , 15,   8,  0,  1,  0,  0,  0,  0),
    /* Granary       */ (Skill::Agriculture , 15,  12,  0,  2,  0,  0,  0,  0),
    /* Herbalist     */ (Skill::Wilderness  , 15,  10,  0,  1,  0,  0,  0,  0),
    /* Inn           */ (Skill::Trade       , 15,  10,  0,  2,  0,  0,  0,  0),
    /* Shrine        */ (Skill::Folklore    , 15,   8,  0,  2,  0,  0,  1,  0),
    /* TavernDive    */ (Skill::Trade       , 15,  12,  0,  1,  0,  0,  0,  0),
    /* WallWooden    */ (Skill::Defense     , 15,   2,  0,  4,  0,  0,  0,  0),
    //                      Skill             DC  RP   FD  LU  LX  OR  ST  More

    //                               Level 2
    //                      Skill             DC  RP   FD  LU  LX  OR  ST  More
    /* Bridge        */ (Skill::Engineering , 16,   6,  0,  0,  0,  0,  0,  1), // 1 Lumber or Stone
    /* Dump          */ (Skill::Industry    , 16,   4,  0,  0,  0,  0,  0,  0),
    /* Jail          */ (Skill::Defense     , 16,  14,  0,  4,  0,  2,  4,  0),
    /* Library       */ (Skill::Scholarship , 16,   6,  0,  4,  0,  0,  2,  0),
    /* Mill          */ (Skill::Industry    , 16,   6,  0,  2,  0,  0,  1,  0),
    /* TownHall      */ (Skill::Industry    ,  0,  22,  0,  4,  0,  0,  4,  0), // TODO: handle "varies" skill (Town Hall -> Castle -> Palace)
    //                      Skill             DC  RP   FD  LU  LX  OR  ST  More

    //                              Level 3
    //                      Skill             DC  RP   FD  LU  LX  OR  ST  More
    /* AlchemyLab    */ (Skill::Industry    , 16,  18,  0,  0,  0,  2,  5,  0),
    /* Barracks      */ (Skill::Defense     , 16,   6,  0,  2,  0,  0,  1,  0),
    /* Keep          */ (Skill::Defense     , 18,  32,  0,  8,  0,  0,  8,  0),
    /* FestivalHall  */ (Skill::Arts        , 18,   7,  0,  3,  0,  0,  0,  0),
    /* Lumberyard    */ (Skill::Industry    , 18,  16,  0,  5,  0,  1,  0,  0),
    /* Monument      */ (Skill::Arts        , 18,   6,  0,  0,  0,  0,  1,  0),
    /* Park          */ (Skill::Wilderness  , 18,   5,  0,  0,  0,  0,  0,  0),
    /* Pier          */ (Skill::Boating     , 18,  16,  0,  2,  0,  0,  0,  0),
    /* Smithy        */ (Skill::Industry    , 18,   8,  0,  2,  0,  1,  1,  0),
    /* Stable        */ (Skill::Wilderness  , 18,  10,  0,  2,  0,  0,  0,  0),
    /* Stockyard     */ (Skill::Industry    , 18,  20,  0,  4,  0,  0,  0,  0),
    /* Stonemason    */ (Skill::Industry    , 18,  16,  0,  2,  0,  0,  0,  0),
    /* Tannery       */ (Skill::Industry    , 18,   6,  0,  2,  0,  0,  0,  0),
    /* TavernPopular */ (Skill::Trade       , 18,  24,  0,  6,  0,  0,  2,  0),
    /* TradeShop     */ (Skill::Trade       , 18,  10,  0,  2,  0,  0,  0,  0),
    /* Watchtower    */ (Skill::Defense     , 18,  12,  0,  0,  0,  0,  0,  4), // 4 Lumber or Stone
    //                      Skill             DC  RP   FD  LU  LX  OR  ST  More
];

#[derive(Debug, Copy, Clone, PartialEq, Eq)] pub struct RpCost(i8);
#[derive(Debug, Copy, Clone, PartialEq, Eq)] pub struct FoodCost(i8);
#[derive(Debug, Copy, Clone, PartialEq, Eq)] pub struct LumberCost(i8);
#[derive(Debug, Copy, Clone, PartialEq, Eq)] pub struct LuxuryCost(i8);
#[derive(Debug, Copy, Clone, PartialEq, Eq)] pub struct OreCost(i8);
#[derive(Debug, Copy, Clone, PartialEq, Eq)] pub struct StoneCost(i8);
#[derive(Debug, Copy, Clone, PartialEq, Eq)] pub struct OtherCommodityCost(i8);


pub fn build_structure(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext, structure: Structure) -> (TurnState, KingdomState) {
    let stats = STRUCTURE_STATS[structure as usize];
    let (skill, dc, rp, food, lumber, luxury, ore, stone, others) = stats;

    build_structure_from_stats(
        kingdom,
        turn,
        state,
        context,
        structure,
        skill,
        DC(dc),
        RpCost(rp),
        FoodCost(food),
        LumberCost(lumber),
        LuxuryCost(luxury),
        OreCost(ore),
        StoneCost(stone),
        OtherCommodityCost(others),
    )
}

fn commodity_cost(degree: DegreeOfSuccess, commodity: i8) -> i8 {
    match degree {
        DegreeOfSuccess::CriticalSuccess => commodity - commodity / 2,  // Get half back
        _                                => commodity,
    }
}

pub fn build_structure_from_stats(
    kingdom: &Kingdom,
    turn: &TurnState,
    state: &KingdomState,
    context: &RollContext,
    // structure building
    structure: Structure,
    skill: Skill,
    dc: DC,
    // costs
    rp_cost: RpCost,
    food_cost: FoodCost,
    lumber_cost: LumberCost,
    luxury_cost: LuxuryCost,
    ore_cost: OreCost,
    stone_cost: StoneCost,
    other_commodity_cost: OtherCommodityCost,
) -> (TurnState, KingdomState)
{
    let the_roll = kingdom.roll(skill, context);

    let degree = roll_result::rate_success(
        the_roll.natural,
        the_roll.total,
        dc,
    );

    let grid_msg = "mark the urban grid with the new stucture".to_string();
    let bonuses_msg = "adjust kingdom item bonuses accordingly".to_string();
    let new_requirements = match degree {
        DegreeOfSuccess::CriticalSuccess => vec![grid_msg, bonuses_msg],
        DegreeOfSuccess::Success         => vec![grid_msg, bonuses_msg],
        _                                => vec![],
    };

    let food_cost  = commodity_cost(degree, food_cost.0);
    let lumber_cost= commodity_cost(degree, lumber_cost.0);
    let luxury_cost= commodity_cost(degree, luxury_cost.0);
    let ore_cost   = commodity_cost(degree, ore_cost.0);
    let stone_cost = commodity_cost(degree, stone_cost.0);

    let mut next_turn_state = turn.clone();
    next_turn_state.requirements.extend(new_requirements);
    if degree == DegreeOfSuccess::Failure {
        assert_eq!(None, next_turn_state.can_build_this_structure_for_no_resource_cost);
        next_turn_state.can_build_this_structure_for_no_resource_cost = Some(structure);
    }
    if degree == DegreeOfSuccess::CriticalFailure {
        next_turn_state.requirements.push("fill the lot(s) in the Urban Grid with rubble".to_string());
    }
    if other_commodity_cost.0 >= 1 {
        next_turn_state.requirements.push("the structure has commodity costs that have not been deducted".to_string());
    }

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.resource_points -= rp_cost.0;

    next_kingdom_state.commodity_stores[Commodity::Food]     -= food_cost;
    next_kingdom_state.commodity_stores[Commodity::Lumber]   -= lumber_cost;
    next_kingdom_state.commodity_stores[Commodity::Luxuries] -= luxury_cost;
    next_kingdom_state.commodity_stores[Commodity::Ore]      -= ore_cost;
    next_kingdom_state.commodity_stores[Commodity::Stone]    -= stone_cost;

    (next_turn_state, next_kingdom_state)
}

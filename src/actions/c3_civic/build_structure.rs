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

type StructureStatsTableRow = (Skill, i8, i8, i8, i8, i8, i8, i8);

const STRUCTURE_STATS: &'static [StructureStatsTableRow] = &[
    // CAUTION: This order must be kept in sync with the definition of Structure above
    //
    //                               Housing
    //                      Skill             DC  RP   FD  LU  LX  OR  ST
    /* Tenement      */ (Skill::Industry    , 14,   1,  0,  1,  0,  0,  0),
    /* Houses        */ (Skill::Industry    , 15,   3,  0,  1,  0,  0,  0),
    /* Orphange      */ (Skill::Industry    , 16,   6,  0,  2,  0,  0,  0),

    //                              Level 1
    //                      Skill             DC  RP   FD  LU  LX  OR  ST
    /* Brewery       */ (Skill::Agriculture , 15,   6,  0,  2,  0,  0,  0),
    /* Cemetery      */ (Skill::Folklore    , 15,   4,  0,  0,  0,  0,  1),
    /* GeneralStore  */ (Skill::Trade       , 15,   8,  0,  1,  0,  0,  0),
    /* Granary       */ (Skill::Agriculture , 15,  12,  0,  2,  0,  0,  0),
    /* Herbalist     */ (Skill::Wilderness  , 15,  10,  0,  1,  0,  0,  0),
    /* Inn           */ (Skill::Trade       , 15,  10,  0,  2,  0,  0,  0),
    /* Shrine        */ (Skill::Folklore    , 15,   8,  0,  0,  0,  0,  3), // FIXME: should be 3 others, not stone
    /* TavernDive    */ (Skill::Trade       , 15,  12,  0,  1,  0,  0,  0),
    /* WallWooden    */ (Skill::Defense     , 15,   2,  0,  4,  0,  0,  0),
    //                      Skill             DC  RP   FD  LU  LX  OR  ST

    //                               Level 2
    //                      Skill             DC  RP   FD  LU  LX  OR  ST
    /* Bridge        */ (Skill::Engineering , 16,   6,  0,  0,  0,  0,  0), //  1 lumber or stone
    /* Dump          */ (Skill::Industry    , 16,   4,  0,  0,  0,  0,  0),
    /* Jail          */ (Skill::Defense     , 16,  14,  0,  0,  0,  0,  0), // 10 others
    /* Library       */ (Skill::Scholarship , 16,   6,  0,  0,  0,  0,  0), //  7 others
    /* Mill          */ (Skill::Industry    , 16,   6,  0,  2,  0,  0,  1),
    /* TownHall      */ (Skill::Industry    ,  0,  22,  0,  0,  0,  0,  0), // TODO: handle "varies"; 8 others
    //                      Skill             DC  RP   FD  LU  LX  OR  ST

    //                              Level 3
    //                      Skill             DC  RP   FD  LU  LX  OR  ST
    /* AlchemyLab    */ (Skill::Industry    , 16,  18,  0,  0,  0,  2,  5), // TODO: verify DC
    /* Barracks      */ (Skill::Defense     , 16,   6,  0,  2,  0,  0,  1), // TODO: verify DC
    /* Keep          */ (Skill::Defense     , 18,  32,  0,  0,  0,  0,  0), // 16 others
    /* FestivalHall  */ (Skill::Arts        , 18,   7,  0,  3,  0,  0,  0),
    /* Lumberyard    */ (Skill::Industry    , 18,  16,  0,  0,  0,  0,  0), //  6 others
    /* Monument      */ (Skill::Arts        , 18,   6,  0,  0,  0,  0,  1),
    /* Park          */ (Skill::Wilderness  , 18,   5,  0,  0,  0,  0,  0),
    /* Pier          */ (Skill::Boating     , 18,  16,  0,  2,  0,  0,  0),
    /* Smithy        */ (Skill::Industry    , 18,   8,  0,  0,  0,  0,  0), //  4 others
    /* Stable        */ (Skill::Wilderness  , 18,  10,  0,  2,  0,  0,  0),
    /* Stockyard     */ (Skill::Industry    , 18,  20,  0,  4,  0,  0,  0),
    /* Stonemason    */ (Skill::Industry    , 18,  16,  0,  2,  0,  0,  0),
    /* Tannery       */ (Skill::Industry    , 18,   6,  0,  2,  0,  0,  0),
    /* TavernPopular */ (Skill::Trade       , 18,  24,  0,  0,  0,  0,  0), //  8 others
    /* TradeShop     */ (Skill::Trade       , 18,  10,  0,  2,  0,  0,  0),
    /* Watchtower    */ (Skill::Defense     , 18,  12,  0,  0,  0,  0,  0), //  8 others
    //                      Skill             DC  RP   FD  LU  LX  OR  ST
];


pub fn build_structure(kingdom: &Kingdom, turn: &TurnState, state: &KingdomState, context: &RollContext, structure: Structure) -> (TurnState, KingdomState) {
    let stats = STRUCTURE_STATS[structure as usize];
    let (skill, dc, rp, food, lumber, luxury, ore, stone) = stats;

    assert_eq!(stone, 3);

    build_structure_from_stats(kingdom, turn, state, context, structure, skill, dc, rp, stone)
}

pub fn build_structure_from_stats(
    kingdom: &Kingdom,
    turn: &TurnState,
    state: &KingdomState,
    context: &RollContext,
    // structure stats:
    structure: Structure,
    skill: Skill,
    dc: i8,
    rp_cost: i8,
    stone_cost: i8,
) -> (TurnState, KingdomState)
{
    let the_roll = kingdom.roll(skill, context);
    let dc = DC(dc); // TODO

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

    let stone_cost = match degree {
        DegreeOfSuccess::CriticalSuccess => stone_cost - stone_cost / 2,  // Get half back
        _                                => stone_cost,
    };

    let mut next_turn_state = turn.clone();
    next_turn_state.requirements.extend(new_requirements);
    if degree == DegreeOfSuccess::Failure {
        assert_eq!(None, next_turn_state.can_build_this_structure_for_no_resource_cost);
        next_turn_state.can_build_this_structure_for_no_resource_cost = Some(structure);
    }
    if degree == DegreeOfSuccess::CriticalFailure {
        next_turn_state.requirements.push("fill the lot(s) in the Urban Grid with rubble".to_string());
    }

    let mut next_kingdom_state = state.clone();
    next_kingdom_state.resource_points -= rp_cost;
    next_kingdom_state.commodity_stores[Commodity::Stone] -= stone_cost;

    (next_turn_state, next_kingdom_state)
}

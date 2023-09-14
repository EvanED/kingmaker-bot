use crate::{rolls::bonus::Bonus, actions::c3_civic::build_structure::Structure};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RandomEventSelectionMethod {
    AdvantageGM,
    AdvantagePlayers,
}

#[derive(Debug, Default, Clone)]
pub struct TurnState {
    // Tracks for now
    pub bonuses: Vec<Bonus>,
    pub requirements: Vec<String>,

    // Tracks for this turn
    pub create_a_masterpiece_attempted: bool,
    // Gives 10 XP if still true at end of turn
    pub supernatural_solution_available: bool,  // TODO: Or count?

    // Tracks for event phase of this turn
    pub random_event_selection_method: Option<RandomEventSelectionMethod>,
    pub dc6_crop_failure_potential_for_x_turns: i8,

    // Tracks for next turn
    pub collected_taxes: bool,
    pub traded_commodities: bool,
    pub bonus_rp: i8,
    pub additional_fame_points_scheduled: i8,
    pub supernatural_solution_blocked_for_x_turns: Option<i8>,
    // FIXME: this should be per-settlement:
    pub can_build_this_structure_for_no_resource_cost: Option<Structure>,
}

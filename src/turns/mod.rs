use crate::rolls::bonus::Bonus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RandomEventSelectionMethod {
    AdvantageGM,
    AdvantagePlayers,
}

#[derive(Debug, Default, Clone)]
pub struct TurnState {
    // Gives 10 XP if still true at end of turn
    pub supernatural_solution_available: bool,  // TODO: Or count?

    pub bonuses: Vec<Bonus>,
    
    pub requirements: Vec<String>,
    
    pub collected_taxes: bool,
    pub traded_commodities: bool,
    pub bonus_rp: i8,
    pub additional_fame_points_scheduled: i8,
    pub supernatural_solution_blocked_for_x_turns: Option<i8>,
    pub create_a_masterpiece_attempted: bool,

    pub random_event_selection_method: Option<RandomEventSelectionMethod>,
    pub dc6_crop_failure_potential_for_x_turns: i8,
}

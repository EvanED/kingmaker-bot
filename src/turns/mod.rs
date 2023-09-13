use crate::rolls::bonus::Bonus;

#[derive(Debug, Default, Clone)]
pub struct TurnState {
    pub supernatural_solution_available: bool,  // TODO: Or count?

    pub bonuses: Vec<Bonus>,
    
    pub requirements: Vec<String>,
    
    pub collected_taxes: bool,
    pub traded_commodities: bool,
    pub bonus_rp: i8,
    pub additional_fame_points_scheduled: i8,
}

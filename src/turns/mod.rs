use crate::rolls::bonus::Bonus;

#[derive(Debug, Default, Clone)]
pub struct TurnState {
    pub collected_taxes: bool,
    pub traded_commodities: bool,
    pub bonuses: Vec<Bonus>,
    pub requirements: Vec<String>,
    pub bonus_rp: i8,
    pub additional_fame_points_scheduled: i8,
}

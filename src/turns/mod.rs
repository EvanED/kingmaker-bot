use crate::rolls::bonus::Bonus;

#[derive(Debug, Default, Clone)]
pub struct TurnState {
    pub collected_taxes: bool,
    pub bonuses: Vec<Bonus>,
}
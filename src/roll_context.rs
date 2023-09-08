use crate::bonus::Bonus;

#[derive(Debug)]
pub struct RollContext {
    pub d20: i8,
    pub bonuses: Vec<Bonus>,
}

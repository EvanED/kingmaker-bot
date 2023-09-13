use super::bonus::Bonus;

#[derive(Debug)]
pub struct RollContext {
    pub d4: i8,
    pub d6: i8,
    pub d20: i8,
    pub bonuses: Vec<Bonus>,
}

use super::bonus::Bonus;

#[derive(Debug, Clone, Copy)]
pub enum RollType {
    FairRoll,
    FixedResult(i8),
}

impl RollType {
    pub fn roll(self) -> i8 {
        match self {
            RollType::FairRoll           => todo!(),
            RollType::FixedResult(x) => x,
        }
    }
}

#[derive(Debug)]
pub struct RollContext {
    pub d4: RollType,
    pub d6: RollType,
    pub d20: RollType,
    pub bonuses: Vec<Bonus>,
}

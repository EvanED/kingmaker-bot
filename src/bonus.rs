#[derive(Debug, Clone, Copy)]
pub enum BonusType {
    Circumstance,
    Item,
    Status,
}

#[derive(Debug)]
pub struct Bonus {
    pub type_: BonusType,
    pub modifier: i8,
    pub reason: String,
}

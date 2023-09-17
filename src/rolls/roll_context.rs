use super::bonus::Bonus;
use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RollType<const FACES: i8> {
    FairRoll,
    FixedResult(i8),
}

fn roll(faces: i8) -> i8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=faces)
}

impl<const FACES: i8> RollType<FACES> {
    pub fn roll(self) -> i8 {
        match self {
            RollType::FairRoll           => roll(FACES),
            RollType::FixedResult(x) => x,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RollContext {
    pub d4: RollType<4>,
    pub d6: RollType<6>,
    pub d20: RollType<20>,
    pub bonuses: Vec<Bonus>,
}

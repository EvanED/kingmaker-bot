use crate::{spec::Kingdom, state::KingdomState, turns::TurnState, rolls::roll_context::RollContext, discord::commands::kingdom::create_aryc};

#[derive(Debug)]
pub struct TurnRecord {
    pub description:   String,
    pub kingdom_state: KingdomState,
    pub turn_state:    TurnState,
}

impl TurnRecord {
    pub fn diff(&self, other: &TurnRecord) -> Vec<String> {
        let mut kingdom_changes = self.kingdom_state.diff(&other.kingdom_state);
        let turn_changes = self.turn_state.diff(&other.turn_state);
        // why can turn_changes not be 'mut'?

        kingdom_changes.extend(turn_changes.into_iter());
        kingdom_changes
    }
}

#[derive(Debug)]
pub struct OverallState {
    pub context: RollContext,
    pub kingdom: Kingdom,
    pub turns: Vec<TurnRecord>,
}

#[derive(Debug)]
pub struct MoveResult {
    pub state_changes: Vec<String>,
}

impl OverallState {
    pub fn new() -> OverallState {
        OverallState {
            context: RollContext {
                d4: 2,
                d6: 6,
                d20: 20,
                bonuses: Vec::new(),
            },
            kingdom: create_aryc(),
            turns: vec![
                TurnRecord {
                    description: "Initial state".to_string(),
                    kingdom_state: KingdomState::default(),
                    turn_state: TurnState::default(),
                }
            ],
        }
    }

    pub fn make_move<F>(&mut self, description: String, turn_func: F) -> MoveResult
        where F: FnOnce(&Kingdom, &TurnState, &KingdomState, &RollContext) -> (TurnState, KingdomState)
    {
        println!("make_move({description}, ...)");
        let starting_state = self.turns.last().unwrap();
    
        let (next_turn_state, next_kingdom_state) = turn_func(
            &self.kingdom,
            &starting_state.turn_state,
            &starting_state.kingdom_state,
            &self.context,
        );
        let next_turn = TurnRecord {
            description,
            kingdom_state: next_kingdom_state,
            turn_state: next_turn_state,
        };
        let state_changes = starting_state.diff(&next_turn);
        self.turns.push(next_turn);

        MoveResult {
            state_changes
        }
    }
}


#[cfg(test)]
pub mod tests {
    use assert2::assert;
    
    #[test]
    fn foo() {
        assert!(1 == 1);
    }
}
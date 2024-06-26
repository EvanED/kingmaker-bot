use serde::{Serialize, Deserialize};

use crate::{spec::Kingdom, state::KingdomState, turns::TurnState, rolls::{roll_context::{RollContext, RollType}, roll_result::RollResult}, discord::commands::kingdom::create_aryc, Markdownable};

#[derive(Debug, Serialize, Deserialize)]
pub struct TurnRecord {
    #[serde(default)]
    pub player:        Option<String>,  // TODO: remove Option
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

impl Markdownable for TurnRecord {
    fn to_markdown(&self) -> String {
        let player = match self.player.as_ref() {
            Some(s) => &s,
            None            => "(unknown)",
        };
        format!("1. *{}:* {}", player, self.description)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverallState {
    pub context: RollContext,
    pub kingdom: Kingdom,
    pub turns: Vec<TurnRecord>,
}

#[derive(Debug)]
pub struct MoveResult {
    pub roll_result: RollResult,
    pub state_changes: Vec<String>,
}

impl MoveResult {
    pub fn to_markdown(&self, roll_description: &str) -> String {
        let degree_str = self.roll_result.degree.to_markdown();
        let total = self.roll_result.die_roll.total.0;
        let working_out = &self.roll_result.die_roll.description;
        let dc = self.roll_result.dc.0;

        let mut text = format!("\
## {roll_description}: {degree_str}

**Total {total} (DC {dc}):** {working_out}");

        for change in &self.state_changes {
            text.push_str("\n* ");
            text.push_str(change);
        }

        text
    }
}

impl OverallState {
    pub fn history_to_markdown(&self) -> String {
        self.turns.iter()
            .map(|turn| turn.to_markdown())
            .filter(
                |desc|
                !desc.contains("GM discharged") && !desc.contains("GM set")
            )
            .collect::<Vec<String>>()
            .join("\n")
    }
    
    pub fn new() -> OverallState {
        OverallState {
            context: RollContext {
                d4: RollType::FairRoll,
                d6: RollType::FairRoll,
                d20: RollType::FairRoll,
                bonuses: Vec::new(),
            },
            kingdom: create_aryc(),
            turns: vec![
                TurnRecord {
                    player: None,
                    description: "Initial state".to_string(),
                    kingdom_state: KingdomState::default(),
                    turn_state: TurnState::default(),
                }
            ],
        }
    }

    pub fn make_move<F>(&mut self, player: &str, description: String, turn_func: F) -> MoveResult
        where F: FnOnce(&Kingdom, &TurnState, &KingdomState, &RollContext) -> (RollResult, TurnState, KingdomState)
    {
        println!("make_move({description}, ...)");
        let starting_state = self.turns.last().unwrap();

        self.context.bonuses = starting_state.turn_state.bonuses.clone();
    
        let (roll_result, next_turn_state, next_kingdom_state) = turn_func(
            &self.kingdom,
            &starting_state.turn_state,
            &starting_state.kingdom_state,
            &self.context,
        );

        let description = format!(
            "{} ({})",
            description,
            roll_result.degree.to_markdown(),
        );

        let next_turn = TurnRecord {
            player: Some(player.to_string()),
            description,
            kingdom_state: next_kingdom_state,
            turn_state: next_turn_state,
        };
        let state_changes = starting_state.diff(&next_turn);
        self.turns.push(next_turn);

        MoveResult {
            roll_result,
            state_changes
        }
    }

    pub fn make_update<F>(&mut self, player: &str, description: String, update_func: F, changer: Box<dyn FnOnce(i8) -> i8>) -> Vec<String>
        where F: FnOnce(&TurnState, &KingdomState, Box<dyn FnOnce(i8) -> i8>) -> (TurnState, KingdomState)
    {
        println!("make_update({description}, ...)");
        let starting_state = self.turns.last().unwrap();

        let (next_turn_state, next_kingdom_state) = update_func(
            &starting_state.turn_state,
            &starting_state.kingdom_state,
            changer,
        );
        let next_turn = TurnRecord {
            player: Some(player.to_string()),
            description,
            kingdom_state: next_kingdom_state,
            turn_state: next_turn_state,
        };
        let state_changes = starting_state.diff(&next_turn);
        self.turns.push(next_turn);

        state_changes
    }

    pub fn make_update_i16<F>(&mut self, player: &str, description: String, update_func: F, changer: Box<dyn FnOnce(i16) -> i16>) -> Vec<String>
        where F: FnOnce(&TurnState, &KingdomState, Box<dyn FnOnce(i16) -> i16>) -> (TurnState, KingdomState)
    {
        println!("make_update({description}, ...)");
        let starting_state = self.turns.last().unwrap();

        let (next_turn_state, next_kingdom_state) = update_func(
            &starting_state.turn_state,
            &starting_state.kingdom_state,
            changer,
        );
        let next_turn = TurnRecord {
            player: Some(player.to_string()),
            description,
            kingdom_state: next_kingdom_state,
            turn_state: next_turn_state,
        };
        let state_changes = starting_state.diff(&next_turn);
        self.turns.push(next_turn);

        state_changes
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
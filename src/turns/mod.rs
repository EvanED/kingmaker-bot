use std::cmp;

use enum_map::EnumMap;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{
    actions::c3_civic::build_structure::Structure,
    diff_utils::{append_bool_change, append_number_change, append_set_change},
    rolls::bonus::Bonus,
    spec::enum_map_serde,
    state::Commodity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RandomEventSelectionMethod {
    AdvantageGM,
    AdvantagePlayers,
}

impl RandomEventSelectionMethod {
    fn to_markdown(self) -> &'static str {
        match self {
            RandomEventSelectionMethod::AdvantageGM => {
                "* A random event is guaranteed. The GM chooses between two random selections"
            }
            RandomEventSelectionMethod::AdvantagePlayers => {
                "* If a random event happens, the players choose between two random selections"
            }
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TurnState {
    // Tracks for now
    pub bonuses: Vec<Bonus>,
    pub requirements: Vec<String>,

    // Tracks for this turn
    pub create_a_masterpiece_attempted: bool,
    // Gives 10 XP if still true at end of turn
    pub supernatural_solution_available: bool, // TODO: Or count?

    // Tracks for event phase of this turn
    pub random_event_selection_method: Option<RandomEventSelectionMethod>,
    pub dc6_crop_failure_potential_for_x_turns: i8,

    // Tracks for next turn
    pub collected_taxes: bool,
    pub traded_commodities: bool,
    pub bonus_rp: i8,
    pub additional_fame_points_scheduled: i8,
    pub supernatural_solution_blocked_for_x_turns: Option<i8>,
    // FIXME: this should be per-settlement:
    pub can_build_this_structure_for_no_resource_cost: Option<Structure>,

    #[serde(with = "enum_map_serde", default)]
    pub commodity_income: EnumMap<Commodity, i8>,

    #[serde(default)]
    pub random_event_dc: i8,
}

fn sub_min_0(x: i8) -> i8 {
    cmp::max(x - 1, 0)
}

fn sub_option(x: Option<i8>) -> Option<i8> {
    match x {
        None => None,
        Some(1) => None,
        Some(x) => Some(x - 1),
    }
}

impl TurnState {
    pub fn next_random_event_dc(&self, was_random_event: bool) -> i8 {
        if was_random_event {
            16i8 // back to default
        }
        else if self.random_event_dc <= 6 {
            2i8 // special case: don't decrease the DC below 2
        }
        else {
            self.random_event_dc - 5
        }
    }

    pub fn next_turn(&self, was_random_event: bool) -> TurnState {
        TurnState {
            bonuses: (self
                .bonuses
                .iter()
                .filter_map(|bonus| bonus.transform_at_turn_start())
                .collect()),
            requirements: self.requirements.clone(),

            // Information tracked for this turn is (mostly) reset
            create_a_masterpiece_attempted: false,
            supernatural_solution_available: false,
            random_event_selection_method: None,

            // Counters get decremented
            dc6_crop_failure_potential_for_x_turns: sub_min_0(
                self.dc6_crop_failure_potential_for_x_turns,
            ),
            supernatural_solution_blocked_for_x_turns: sub_option(
                self.supernatural_solution_blocked_for_x_turns,
            ),

            // Gets reset right away, but contributes (TODO) to the KingdomState
            additional_fame_points_scheduled: 0,

            // These will be reset very early in the turn, but not yet:
            collected_taxes: self.collected_taxes,
            traded_commodities: self.traded_commodities,
            bonus_rp: 0,

            // These could in theory carry forward indefinitely
            can_build_this_structure_for_no_resource_cost: self
                .can_build_this_structure_for_no_resource_cost,
            commodity_income: self.commodity_income.clone(),

            random_event_dc: self.next_random_event_dc(was_random_event),
        }
    }

    pub fn diff(&self, other: &TurnState) -> Vec<String> {
        let mut diffs = Vec::new();

        append_set_change(&mut diffs, "bonus", &self.bonuses, &other.bonuses);
        append_set_change(
            &mut diffs,
            "requirement",
            &self.requirements,
            &other.requirements,
        );

        append_bool_change(
            &mut diffs,
            self.collected_taxes,
            "'Collected Taxes' was reset",
            other.collected_taxes,
            "The kingdom collected taxes this action",
        );

        append_bool_change(
            &mut diffs,
            self.supernatural_solution_available,
            "Supernatural Solution's substitution was used",
            other.supernatural_solution_available,
            "Supernatural Solution's substitution became available",
        );

        for commodity in Commodity::iter() {
            append_number_change(
                &mut diffs,
                &format!("{} income", commodity),
                self.commodity_income[commodity],
                other.commodity_income[commodity],
            );
        }

        append_number_change(
            &mut diffs,
            "Random event DC",
            self.random_event_dc,
            other.random_event_dc,
        );

        append_number_change(
            &mut diffs,
            "Bonus RP for next turn",
            self.bonus_rp,
            other.bonus_rp,
        );

        diffs
    }

    fn next_turn_info(&self) -> String {
        let mut strings: Vec<String> = Vec::new();

        if self.collected_taxes {
            strings.push("* We collected taxes this turn".to_string());
        }
        if self.traded_commodities {
            strings.push("* We traded commodities this turn".to_string());
        }
        if self.bonus_rp > 0 {
            strings.push(format!(
                "* The kingdom will receive {} bonus RP",
                self.bonus_rp
            ));
        }
        if self.additional_fame_points_scheduled > 0 {
            strings.push(format!(
                "* The kingdom will receive {} bonus Fame points",
                self.additional_fame_points_scheduled,
            ));
        }
        if self.supernatural_solution_blocked_for_x_turns.is_some() {
            strings.push(format!(
                "* Supernatural Solution is blocked for {} turns",
                self.supernatural_solution_blocked_for_x_turns.unwrap(),
            ));
        }
        if self.can_build_this_structure_for_no_resource_cost.is_some() {
            strings.push(
                format!(
                    "* The structure {} is in progress and building does not require sepending resources again",
                    self.can_build_this_structure_for_no_resource_cost.unwrap().as_static_str(),
                )
            );
        }

        if strings.is_empty() {
            return "".to_string();
        } else {
            strings.insert(0, "Information for future turns:".to_string());
            strings.join("\n")
        }
    }

    fn this_turn_info(&self) -> String {
        let mut strings: Vec<String> = Vec::new();

        if self.create_a_masterpiece_attempted {
            strings.push("* Create a Masterpiece has been attempted".to_string());
        }
        if self.supernatural_solution_available {
            strings.push("* Supernatural Solution's success condition may be used".to_string())
        }
        if self.random_event_selection_method.is_some() {
            strings.push(
                self.random_event_selection_method
                    .unwrap()
                    .to_markdown()
                    .to_string(),
            );
        }
        if self.dc6_crop_failure_potential_for_x_turns > 0 {
            strings.push(format!(
                "* For {} turns, on a DC 6 flat check failure a Crop Failure event occurs",
                self.dc6_crop_failure_potential_for_x_turns,
            ));
        }

        strings.push(
            format!("* Random kingdom event DC: {}", self.random_event_dc)
        );

        strings.insert(0, "This turn:".to_string());
        strings.join("\n")
    }

    fn bonuses_markdown_yes(&self) -> String {
        let mut s = "Bonuses:".to_string();
        for bonus in &self.bonuses {
            s.push_str("\n1. ");
            bonus.append_markdown(&mut s);
        }
        s
    }

    fn bonuses_markdown(&self) -> String {
        if self.bonuses.is_empty() {
            "No bonuses/penalties  ".to_string()
        } else {
            self.bonuses_markdown_yes()
        }
    }

    fn requirements_markdown_yes(&self) -> String {
        let mut s = "Requirements:".to_string();
        for request in &self.requirements {
            s.push_str("\n1. ");
            s.push_str(request);
        }
        s
    }

    fn requirements_markdown(&self) -> String {
        if self.requirements.is_empty() {
            "No requirements  ".to_string()
        } else {
            self.requirements_markdown_yes()
        }
    }

    fn commodity_income_string(&self) -> String {
        format!(
            "**Commodity Income:** Food {}, Lumber {}, Luxuries {}, Ore {}, Stone {}",
            self.commodity_income[Commodity::Food],
            self.commodity_income[Commodity::Lumber],
            self.commodity_income[Commodity::Luxuries],
            self.commodity_income[Commodity::Ore],
            self.commodity_income[Commodity::Stone],
        )
    }

    pub fn to_markdown(&self) -> String {
        let commodity_income_string = self.commodity_income_string();
        let bonuses_string = self.bonuses_markdown();
        let requirements_string = self.requirements_markdown();
        let this_turn_string = self.this_turn_info();
        let next_turn_string = self.next_turn_info();

        format!(
            "
## Current Turn State

{commodity_income_string}
{bonuses_string}
{requirements_string}
{this_turn_string}
{next_turn_string}
            "
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        rolls::bonus::{AppliesTo, AppliesUntil, BonusType},
        spec::skills::Skill,
    };

    use super::*;
    use assert2::assert;
    use enum_map::enum_map;

    #[test]
    fn commodity_income_changes_reflected_in_text() {
        let mut k1 = TurnState::default();
        let mut k2 = TurnState::default();

        k1.commodity_income[Commodity::Ore] = 4;
        k2.commodity_income[Commodity::Ore] = 2;

        let diff = k1.diff(&k2);
        assert!(diff == vec!["Ore income decreased from 4 to 2",]);
    }

    #[test]
    fn random_event_dc_change_reflected_in_text() {
        let mut k1 = TurnState::default();
        let mut k2 = TurnState::default();

        k1.random_event_dc = 16;
        k2.random_event_dc = 11;

        let diff = k1.diff(&k2);
        assert!(diff == vec!["Random event DC decreased from 16 to 11",]);
    }

    #[test]
    fn bonus_rp_change_reflected_in_text() {
        let mut k1 = TurnState::default();
        let mut k2 = TurnState::default();

        k1.bonus_rp = 10;
        k2.bonus_rp = 15;

        let diff = k1.diff(&k2);
        assert!(diff == vec!["Bonus RP for next turn increased from 10 to 15",]);
    }

    #[test]
    fn bonus_rp_reset_at_start_of_turn() {
        let mut turn_state = TurnState::default();
        turn_state.bonus_rp = 7;

        let turn_state = turn_state; // remove mutability
        let next_turn_state = turn_state.next_turn(true);

        assert!(next_turn_state.bonus_rp == 0);
    }

    #[test]
    fn bonus_and_requirement_changes_reflected_in_text() {
        let mut k1 = TurnState::default();
        let mut k2 = TurnState::default();

        k1.bonuses.push(Bonus {
            type_: BonusType::Circumstance,
            applies_to: AppliesTo::RandomEventResolutions,
            applies_until: AppliesUntil::NextApplicableRoll,
            modifier: 1,
            reason: "dummy".to_string(),
        });
        k2.bonuses.push(Bonus {
            type_: BonusType::Status,
            applies_to: AppliesTo::Skill(Skill::Agriculture),
            applies_until: AppliesUntil::EndOfTheNextTurn,
            modifier: 7,
            reason: "different".to_string(),
        });

        k1.requirements.push("abc".to_string());
        k2.requirements.push("def".to_string());

        let diff = k1.diff(&k2);
        assert!(
            diff == vec![
                "Lost bonus: +1 Circumstance bonus to random event rolls rolls until the next such roll, because dummy",
                "Gained bonus: +7 Status bonus to agriculture rolls until the end of the next kingdom turn, because different",
                "Lost requirement: abc",
                "Gained requirement: def",
            ]
        );
    }

    #[test]
    fn current_turn_tracking() {
        let mut k1 = TurnState::default();
        let mut k2 = TurnState::default();

        k1.collected_taxes = false;
        k2.collected_taxes = true;

        k1.supernatural_solution_available = true;
        k2.supernatural_solution_available = false;

        let diff = k1.diff(&k2);
        assert!(
            diff == vec![
                "The kingdom collected taxes this action",
                "Supernatural Solution's substitution was used",
            ]
        );
    }

    fn create_test_turn_state() -> TurnState {
        TurnState {
            bonuses: vec![],
            requirements: vec!["requirement #1".to_string(), "requirement #2".to_string()],
            create_a_masterpiece_attempted: true,
            supernatural_solution_available: true,
            dc6_crop_failure_potential_for_x_turns: 0,
            random_event_selection_method: Some(RandomEventSelectionMethod::AdvantageGM),
            collected_taxes: true,
            traded_commodities: true,
            bonus_rp: 5,
            additional_fame_points_scheduled: 2,
            supernatural_solution_blocked_for_x_turns: None,
            can_build_this_structure_for_no_resource_cost: None,
            commodity_income: enum_map! {
                Commodity::Food     => 1,
                Commodity::Lumber   => 2,
                Commodity::Luxuries => 0,
                Commodity::Ore      => 0,
                Commodity::Stone    => 0,
            },
            random_event_dc: 16,
        }
    }

    #[test]
    fn check_new_turn_gets_dc_correct() {
        let mut start_state = create_test_turn_state();
        start_state.random_event_dc = 11;

        let start_state = start_state; // remove mutation capability

        let next_state_reset_dc = start_state.next_turn(true);
        let next_state_progressive_dc = start_state.next_turn(false);

        assert!(next_state_reset_dc.random_event_dc == 16);
        assert!(next_state_progressive_dc.random_event_dc == 6);
    }

    #[test]
    fn check_new_turn_does_not_decrease_random_event_dc_below_two() {
        let mut start_state = create_test_turn_state();
        start_state.random_event_dc = 6;

        let start_state = start_state; // remove mutation capability
        let next_state_progressive_dc = start_state.next_turn(false);

        assert!(next_state_progressive_dc.random_event_dc == 2);
    }

    #[test]
    fn check_new_turn_gets_basic_stuff_right() {
        let start_state = create_test_turn_state();
        let next_turn = start_state.next_turn(false);

        // Stuff about upcoming turn is reset
        assert!(next_turn.create_a_masterpiece_attempted == false);
        assert!(next_turn.supernatural_solution_available == false);
        assert!(next_turn.random_event_selection_method == None);

        // This should be decremented -- more tests below
        assert!(next_turn.dc6_crop_failure_potential_for_x_turns == 0); // -1

        // Stuff from the previous turn that affects this turn still needs to be tracked
        assert!(next_turn.collected_taxes == true); // same
        assert!(next_turn.traded_commodities == true); // same

        // This gets reset
        assert!(next_turn.bonus_rp == 0);

        // This is applied immediately
        assert!(next_turn.additional_fame_points_scheduled == 0); // CHECK THAT fame == 1 + this

        // These just carry forward
        assert!(next_turn.supernatural_solution_blocked_for_x_turns == None); // -1
        assert!(next_turn.can_build_this_structure_for_no_resource_cost == None);
        assert!(next_turn.requirements == vec!["requirement #1", "requirement #2",]);
    }

    #[test]
    fn check_d6_crop_failure_potential_gets_decremented() {
        let start_state = TurnState {
            dc6_crop_failure_potential_for_x_turns: 5,
            ..create_test_turn_state()
        };
        let next_turn = start_state.next_turn(false);

        assert!(next_turn.dc6_crop_failure_potential_for_x_turns == 4);
    }

    #[test]
    fn check_supernatural_solution_blocked_gets_decremented() {
        let start_state = TurnState {
            supernatural_solution_blocked_for_x_turns: Some(5),
            ..create_test_turn_state()
        };
        let next_turn = start_state.next_turn(false);

        assert!(next_turn.supernatural_solution_blocked_for_x_turns == Some(4));
    }

    #[test]
    fn check_supernatural_solution_blocked_gets_decremented_to_none_instead_of_zero() {
        let start_state = TurnState {
            supernatural_solution_blocked_for_x_turns: Some(1),
            ..create_test_turn_state()
        };
        let next_turn = start_state.next_turn(false);

        assert!(next_turn.supernatural_solution_blocked_for_x_turns == None);
    }

    #[test]
    fn check_bonuses_continue_as_appropriate() {
        let bonuses = vec![
            Bonus {
                type_: BonusType::Circumstance,
                applies_to: AppliesTo::Skill(Skill::Arts),
                applies_until: AppliesUntil::EndOfTheNextTurn,
                modifier: 1,
                reason: "Should Remain Until End of *This* Turn #1".to_string(),
            },
            Bonus {
                type_: BonusType::Circumstance,
                applies_to: AppliesTo::Skill(Skill::Arts),
                applies_until: AppliesUntil::NextApplicableRoll,
                modifier: 1,
                reason: "Should Expire #1".to_string(),
            },
            Bonus {
                type_: BonusType::Circumstance,
                applies_to: AppliesTo::Skill(Skill::Arts),
                applies_until: AppliesUntil::StartOfTheNextTurn,
                modifier: 1,
                reason: "Should Expire #2".to_string(),
            },
            Bonus {
                type_: BonusType::Circumstance,
                applies_to: AppliesTo::Skill(Skill::Arts),
                applies_until: AppliesUntil::EndOfTheNextTurn,
                modifier: 1,
                reason: "Should Remain Until End of *This* Turn #2".to_string(),
            },
        ];

        let start_state = TurnState {
            bonuses,
            ..create_test_turn_state()
        };
        let next_turn = start_state.next_turn(false);

        let expected_bonuses = vec![
            Bonus {
                type_: BonusType::Circumstance,
                applies_to: AppliesTo::Skill(Skill::Arts),
                // was EndOfTheNextTurn
                applies_until: AppliesUntil::StartOfTheNextTurn,
                modifier: 1,
                reason: "Should Remain Until End of *This* Turn #1".to_string(),
            },
            Bonus {
                type_: BonusType::Circumstance,
                applies_to: AppliesTo::Skill(Skill::Arts),
                // was EndOfTheNextTurn
                applies_until: AppliesUntil::StartOfTheNextTurn,
                modifier: 1,
                reason: "Should Remain Until End of *This* Turn #2".to_string(),
            },
        ];

        assert!(next_turn.bonuses == expected_bonuses);
    }
}

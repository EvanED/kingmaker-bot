use crate::{rolls::bonus::Bonus, actions::c3_civic::build_structure::Structure};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RandomEventSelectionMethod {
    AdvantageGM,
    AdvantagePlayers,
}

impl RandomEventSelectionMethod {
    fn to_markdown(self) -> &'static str {
        match self {
            RandomEventSelectionMethod::AdvantageGM      => "* A random event is guaranteed. The GM chooses between two random selections",
            RandomEventSelectionMethod::AdvantagePlayers => "* If a random event happens, the players choose between two random selections",
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct TurnState {
    // Tracks for now
    pub bonuses: Vec<Bonus>,
    pub requirements: Vec<String>,

    // Tracks for this turn
    pub create_a_masterpiece_attempted: bool,
    // Gives 10 XP if still true at end of turn
    pub supernatural_solution_available: bool,  // TODO: Or count?

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
}

impl TurnState {
    fn next_turn_info(&self) -> String {
        let mut strings: Vec<String> = Vec::new();

        if self.collected_taxes {
            strings.push("* We collected taxes this turn".to_string());
        }
        if self.traded_commodities {
            strings.push("* We traded commodities this turn".to_string());
        }
        if self.bonus_rp > 0 {
            strings.push(
                format!("* The kingdom will receive {} bonus RP", self.bonus_rp)
            );
        }
        if self.additional_fame_points_scheduled > 0 {
            strings.push(
                format!(
                    "* The kingdom will receive {} bonus Fame points",
                    self.additional_fame_points_scheduled,
                )
            );
        }
        if self.supernatural_solution_blocked_for_x_turns.is_some() {
            strings.push(
                format!(
                    "* Supernatural Solution is blocked for {} turns",
                    self.supernatural_solution_blocked_for_x_turns.unwrap(),
                )
            );
        }
        if self.can_build_this_structure_for_no_resource_cost.is_some() {
            strings.push(
                format!(
                    "* The structure {} is in progress and building does not require sepending resources again",
                    self.can_build_this_structure_for_no_resource_cost.unwrap().as_ref(),
                )
            );
        }

        if strings.is_empty() {
            return "".to_string();
        }
        else {
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
            strings.push(self.random_event_selection_method.unwrap().to_markdown().to_string());
        }
        if self.dc6_crop_failure_potential_for_x_turns > 0 {
            strings.push(
                format!(
                    "* For {} turns, on a DC 6 flat check failure a Crop Failure event occurs",
                    self.dc6_crop_failure_potential_for_x_turns,
                )
            );
        }

        if strings.is_empty() {
            return "".to_string();
        }
        else {
            strings.insert(0, "This turn:".to_string());
            strings.join("\n")
        }
    }


    fn bonuses_markdown_yes(&self) -> String {
        let mut s = "Bonuses:".to_string();
        for bonus in &self.bonuses {
            s.push_str("\n* ");
            bonus.to_markdown(&mut s);
        };
        s
    }

    fn bonuses_markdown(&self) -> String {
        if self.bonuses.is_empty() {
            "No bonuses/penalties  ".to_string()
        }
        else {
            self.bonuses_markdown_yes()
        }
    }


    fn requirements_markdown_yes(&self) -> String {
        let mut s = "Requirements:".to_string();
        for request in &self.requirements {
            s.push_str("\n* ");
            s.push_str(request);
        };
        s
    }

    fn requirements_markdown(&self) -> String {
        if self.bonuses.is_empty() {
            "No requirements  ".to_string()
        }
        else {
            self.requirements_markdown_yes()
        }
    }


    pub fn to_markdown(&self) -> String {
        let bonuses_string = self.bonuses_markdown();
        let requirements_string = self.requirements_markdown();
        let this_turn_string = self.this_turn_info();
        let next_turn_string = self.next_turn_info();

        format!(
            "
## Current Turn State

{bonuses_string}
{requirements_string}
{this_turn_string}
{next_turn_string}
            "
        )
    }
}
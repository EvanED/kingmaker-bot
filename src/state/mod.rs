use core::panic;

use enum_map::{EnumMap, Enum};
use poise::ChoiceParameter;
use serde::{Serialize, Deserialize};
use strum_macros::{EnumIter, AsRefStr};
use strum::IntoEnumIterator;
use crate::{diff_utils::append_number_change, rolls::roll_result::DC, spec::{attributes::Ruin, enum_map_serde, Kingdom}, turns::TurnState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum, AsRefStr, /*EnumString,*/ EnumIter, Serialize, Deserialize, ChoiceParameter)]
#[strum(ascii_case_insensitive)]
pub enum Commodity {
    Food,
    Lumber,
    Luxuries,
    Ore,
    Stone,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HexCoordinate {
    pub x: i8,
    pub y: i8,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct KingdomState {
    #[serde(default)]
    pub size: i8,
    #[serde(default)]
    pub xp: i16,
    pub unrest: i8,
    pub resource_points: i8,  // More?
    pub fame_points: i8,
    #[serde(with="enum_map_serde")]
    pub commodity_stores: EnumMap<Commodity, i8>,
    #[serde(default)]
    pub claimed_hexes: Vec<HexCoordinate>,
    #[serde(with="enum_map_serde", default)]
    pub ruins: EnumMap<Ruin, i8>,
}

fn level_to_raw_control_dc(level: i8) -> i8 {
    match level {
        1  => 14,
        2  => 15,
        3  => 16,
        4  => 18,
        5  => 20,
        6  => 22,
        7  => 23,
        8  => 24,
        9  => 26,
        10 => 27,
        11 => 28,
        12 => 30,
        13 => 31,
        14 => 32,
        15 => 34,
        16 => 35,
        17 => 36,
        18 => 38,
        19 => 39,
        20 => 40,
        _  => panic!("Bad level"),
    }
}

#[derive(Debug, Clone)]
enum KingdomSizeCategory {
    Territory,
    Province,
    State,
    Country,
    Dominion,
}

fn size_to_category(size: i8) -> KingdomSizeCategory {
    use KingdomSizeCategory::*;
    match size {
        0..=9         => Territory,
        10..=24       => Province,
        25..=49       => State,
        50..=99       => Country,
        100..=i8::MAX => Dominion,
        i8::MIN..=-1  => panic!("You're gonna need a bigger kingdom"),
    }
}

fn size_to_control_dc_modifier(size: KingdomSizeCategory) -> i8 {
    use KingdomSizeCategory::*;
    match size {
        Territory  => 0,
        Province   => 1,
        State      => 2,
        Country    => 3,
        Dominion   => 4,
    }
}

fn xp_gain_from_rp(kingdom: &Kingdom, rp: i8) -> i16 {
    let gain_per_rp = match kingdom.level {
        1..=4   => 10i16,
        5..=8   =>  7i16,
        9..=12  =>  5i16,
        13..=16 =>  2i16,
        17..=20 =>  1i16,
        _       => panic!("Bad kingdom level"), 
    };
    return gain_per_rp * (rp as i16);
}

impl KingdomState {
    pub fn control_dc(&self, kingdom: &Kingdom) -> DC {
        let pre_dc = level_to_raw_control_dc(kingdom.level);

        let size = size_to_category(self.size);
        let dc_mod = size_to_control_dc_modifier(size);

        DC(pre_dc + dc_mod)
    }

    pub fn max_commodity_store(_c: Commodity) -> i8 {
        4
    }

    pub fn next_turn(&self, kingdom: &Kingdom, turn_state: &TurnState) -> KingdomState {
        let mut next_kstate = self.clone();

        for commodity in Commodity::iter() {
            next_kstate.commodity_stores[commodity] += turn_state.commodity_income[commodity];
        }

        next_kstate.fame_points = 1i8 + turn_state.additional_fame_points_scheduled;
        if next_kstate.fame_points > 3i8 {
            next_kstate.fame_points = 3i8;
        }

        next_kstate.resource_points = turn_state.bonus_rp;

        next_kstate.xp += xp_gain_from_rp(kingdom, self.resource_points);

        next_kstate
    }

    pub fn diff(&self, other: &KingdomState) -> Vec<String> {
        let mut diffs = Vec::new();

        append_number_change(&mut diffs, "Size", self.size, other.size);
        append_number_change(&mut diffs, "XP", self.xp, other.xp);
        append_number_change(&mut diffs, "Unrest", self.unrest, other.unrest);

        for ruin in Ruin::iter() {
            append_number_change(
                &mut diffs,
                ruin.into(),
                self.ruins[ruin],
                other.ruins[ruin],
            );
        }

        append_number_change(&mut diffs, "RP", self.resource_points, other.resource_points);
        append_number_change(&mut diffs, "Fame", self.fame_points, other.fame_points);

        for commodity in Commodity::iter() {
            append_number_change(
                &mut diffs,
                commodity.as_ref(),
                self.commodity_stores[commodity],
                other.commodity_stores[commodity],
            );
        }

        diffs
    }

    pub fn hexes_to_markdown(&self) -> String {
        let claimed_str = self.claimed_hexes.iter().map(
            |hex_coord| {
                format!("* {}, {}", hex_coord.x, hex_coord.y)
            }
        )
        .collect::<Vec<String>>()
        .join("\n");
        format!("**Claimed Hexes:**\n{claimed_str}")
    }

    pub fn to_markdown(&self) -> String {
        format!(
            "
## Current Kingdom State

**Size:** {}  \n\
**XP:** {}  \n\
**Unrest:** {}  \n\
**Ruins:** {}  \n\
**Resource Points:** {}  \n\
**Fame Points:** {}
**Commodities:** Food {}, Lumber {}, Luxuries {}, Ore {}, Stone {}
            ",
            self.size,
            self.xp,
            self.unrest,
            self.ruins_to_markdown(),
            self.resource_points,
            self.fame_points,
            self.commodity_stores[Commodity::Food],
            self.commodity_stores[Commodity::Lumber],
            self.commodity_stores[Commodity::Luxuries],
            self.commodity_stores[Commodity::Ore],
            self.commodity_stores[Commodity::Stone],
        )
    }

    pub fn ruins_to_markdown(&self) -> String {
        format!(
            "Corruption: {}, Crime: {}, Strife: {}, Decay: {}",
            self.ruins[Ruin::Corruption],
            self.ruins[Ruin::Crime],
            self.ruins[Ruin::Strife],
            self.ruins[Ruin::Decay],
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::discord::commands::kingdom::create_aryc;

    use core::panic; // deal with some Rust silliness
    use super::*;
    use assert2::assert;

    #[test]
    fn rp_is_set_to_bonus_rp_at_start_of_turn() {
        let aryc = create_aryc();

        let mut k1 = KingdomState::default();
        k1.resource_points = 7;

        let mut turn_state = TurnState::default();
        turn_state.bonus_rp = 10;

        let k2 = k1.next_turn(&aryc, &turn_state);

        assert!(k2.resource_points == 10);
    }

    #[test]
    fn xp_is_increased_according_to_rp_levels_1_to_4() {
        for level in 1..5 {
            let mut aryc = create_aryc();
            aryc.level = level;

            let mut k1 = KingdomState::default();
            k1.xp = 100;
            k1.resource_points = 7;

            let turn_state = TurnState::default();

            let k2 = k1.next_turn(&aryc, &turn_state);

            // RAW is always 1 XP per RP
            assert!(k2.xp == 170); // 10 XP per RP
        }
    }

    #[test]
    fn xp_is_increased_according_to_rp_levels_5_to_8() {
        for level in 5..9 {
            let mut aryc = create_aryc();
            aryc.level = level;

            let mut k1 = KingdomState::default();
            k1.xp = 100;
            k1.resource_points = 7;

            let turn_state = TurnState::default();

            let k2 = k1.next_turn(&aryc, &turn_state);

            // RAW is always 1 XP per rp
            assert!(k2.xp == 149); // 7 XP per RP
        }
    }

    #[test]
    fn xp_is_increased_according_to_rp_levels_9_to_12() {
        for level in 9..13 {
            let mut aryc = create_aryc();
            aryc.level = level;

            let mut k1 = KingdomState::default();
            k1.xp = 100;
            k1.resource_points = 7;

            let turn_state = TurnState::default();

            let k2 = k1.next_turn(&aryc, &turn_state);

            // RAW is always 1 XP per rp
            assert!(k2.xp == 135); // 5 XP per RP
        }
    }

    #[test]
    fn xp_is_increased_according_to_rp_levels_13_to_16() {
        for level in 13..17 {
            let mut aryc = create_aryc();
            aryc.level = level;

            let mut k1 = KingdomState::default();
            k1.xp = 100;
            k1.resource_points = 7;

            let turn_state = TurnState::default();

            let k2 = k1.next_turn(&aryc, &turn_state);

            // RAW is always 1 XP per rp
            assert!(k2.xp == 114); // 2 XP per RP
        }
    }

    #[test]
    fn xp_is_increased_according_to_rp_levels_17_to_20() {
        for level in 17..21 {
            let mut aryc = create_aryc();
            aryc.level = level;

            let mut k1 = KingdomState::default();
            k1.xp = 100;
            k1.resource_points = 7;

            let turn_state = TurnState::default();

            let k2 = k1.next_turn(&aryc, &turn_state);

            // RAW is always 1 XP per rp
            assert!(k2.xp == 107); // 1 XP per RP
        }
    }



    #[test]
    fn commodity_stores_increase_at_start_of_turn() {
        let aryc = create_aryc();

        let mut k1 = KingdomState::default();
        k1.commodity_stores[Commodity::Food] = 1;

        let mut turn_state = TurnState::default();
        turn_state.commodity_income[Commodity::Food] = 1;
        turn_state.commodity_income[Commodity::Ore] = 3;

        //
        let k2 = k1.next_turn(&aryc, &turn_state);

        //
        assert!(k2.commodity_stores[Commodity::Food] == 1 + 1);
        assert!(k2.commodity_stores[Commodity::Ore]  == 0 + 3);
    }

    #[test]
    fn fame_points_reset_at_start_of_turn_from_0() {
        let aryc = create_aryc();

        let mut k1 = KingdomState::default();
        k1.fame_points = 0i8;

        let turn_state = TurnState::default();
        let k2 = k1.next_turn(&aryc, &turn_state);

        assert!(k2.fame_points == 1i8);
    }

    #[test]
    fn fame_points_reset_at_start_of_turn_from_2() {
        let aryc = create_aryc();

        let mut k1 = KingdomState::default();
        k1.fame_points = 2i8;

        let turn_state = TurnState::default();
        let k2 = k1.next_turn(&aryc, &turn_state);

        assert!(k2.fame_points == 1i8);
    }

    #[test]
    fn fame_points_reset_at_start_of_turn_increased_by_bonus() {
        let aryc = create_aryc();

        let mut k1 = KingdomState::default();
        k1.fame_points = 0i8;

        let mut turn_state = TurnState::default();
        turn_state.additional_fame_points_scheduled = 1i8;
        let k2 = k1.next_turn(&aryc, &turn_state);

        assert!(k2.fame_points == 2i8);
    }

    #[test]
    fn fame_points_reset_at_start_of_turn_increased_by_bonus_caps_at_3() {
        let aryc = create_aryc();

        let mut k1 = KingdomState::default();
        k1.fame_points = 0i8;

        let mut turn_state = TurnState::default();
        turn_state.additional_fame_points_scheduled = 10i8;
        let k2 = k1.next_turn(&aryc, &turn_state);

        assert!(k2.fame_points == 3i8);
    }

    #[test]
    fn xp_increase_reflected_in_diff() {
        let k1 = KingdomState {
            xp: 100,
            ..KingdomState::default()
        };
        let k2 = KingdomState {
            xp: 200,
            ..KingdomState::default()
        };
        let diff = k1.diff(&k2);
        assert!(
            diff == vec![
                "XP increased from 100 to 200",
            ]
        );
    }

    #[test]
    fn unrest_decrease_reflected_in_diff() {
        let k1 = KingdomState {
            unrest: 2,
            ..KingdomState::default()
        };
        let k2 = KingdomState {
            unrest: 1,
            ..KingdomState::default()
        };
        let diff = k1.diff(&k2);
        assert!(
            diff == vec![
                "Unrest decreased from 2 to 1",
            ]
        );
    }

    #[test]
    fn unrest_increase_reflected_in_diff() {
        let k1 = KingdomState {
            unrest: 1,
            ..KingdomState::default()
        };
        let k2 = KingdomState {
            unrest: 2,
            ..KingdomState::default()
        };
        let diff = k1.diff(&k2);
        assert!(
            diff == vec![
                "Unrest increased from 1 to 2",
            ]
        );
    }

    #[test]
    fn rp_decrease_reflected_in_diff() {
        let k1 = KingdomState {
            resource_points: 2,
            ..KingdomState::default()
        };
        let k2 = KingdomState {
            resource_points: 1,
            ..KingdomState::default()
        };
        let diff = k1.diff(&k2);
        assert!(
            diff == vec![
                "RP decreased from 2 to 1",
            ]
        );
    }

    #[test]
    fn fame_decrease_reflected_in_diff() {
        let k1 = KingdomState {
            fame_points: 2,
            ..KingdomState::default()
        };
        let k2 = KingdomState {
            fame_points: 1,
            ..KingdomState::default()
        };
        let diff = k1.diff(&k2);
        assert!(
            diff == vec![
                "Fame decreased from 2 to 1",
            ]
        );
    }

    #[test]
    fn food_decrease_reflected_in_diff() {
        let mut k1 = KingdomState::default();
        let mut k2 = KingdomState::default();

        k1.commodity_stores[Commodity::Food] = 2;
        k2.commodity_stores[Commodity::Food] = 1;

        let diff = k1.diff(&k2);
        assert!(
            diff == vec![
                "Food decreased from 2 to 1",
            ]
        );
    }

    #[test]
    fn multiple_changes() {
        let mut k1 = KingdomState::default();
        let mut k2 = KingdomState::default();

        k1.size = 5;
        k2.size = 6;

        k1.unrest = 3;
        k2.unrest = 4;

        k1.commodity_stores[Commodity::Food] = 2;
        k2.commodity_stores[Commodity::Food] = 1;

        k1.commodity_stores[Commodity::Ore] = 5;

        k2.commodity_stores[Commodity::Stone] = 7;

        let diff = k1.diff(&k2);
        assert!(
            diff == vec![
                "Size increased from 5 to 6",
                "Unrest increased from 3 to 4",
                "Food decreased from 2 to 1",
                "Ore decreased from 5 to 0",
                "Stone increased from 0 to 7",
            ]
        );
    }

    #[test]
    fn ruin_changes_reflected_in_diff() {
        let mut k1 = KingdomState::default();
        let mut k2 = KingdomState::default();

        k1.ruins[Ruin::Corruption] = 2;
        k2.ruins[Ruin::Decay] = 2;

        let diff = k1.diff(&k2);
        assert!(
            diff == vec![
                // TODO: capitalize
                "corruption decreased from 2 to 0",
                "decay increased from 0 to 2",
            ]
        );
    }
}
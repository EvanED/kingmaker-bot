use enum_map::{EnumMap, Enum};
use strum_macros::{EnumString, EnumIter, AsRefStr};
use strum::IntoEnumIterator;
use crate::diff_utils::append_number_change;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum, AsRefStr, EnumString, EnumIter)]
#[strum(ascii_case_insensitive)]
pub enum Commodity {
    Food,
    Lumber,
    Luxuries,
    Ore,
    Stone,
}

#[derive(Debug, Default, Clone)]
pub struct KingdomState {
    pub unrest: i8,
    pub resource_points: i8,  // More?
    pub fame_points: i8,
    pub commodity_stores: EnumMap<Commodity, i8>,
}

impl KingdomState {
    pub fn diff(&self, other: &KingdomState) -> Vec<String> {
        let mut diffs = Vec::new();

        append_number_change(&mut diffs, "Unrest", self.unrest, other.unrest);
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

    pub fn to_markdown(&self) -> String {
        format!(
            "
## Current Kingdom State

**Unrest:** {}  \n\
**Resource Points:** {}  \n\
**Fame Points:** {}
**Commodities:** Food {}, Lumber {}, Luxuries {}, Ore {}, Stone {}
            ",
            self.unrest,
            self.resource_points,
            self.fame_points,
            self.commodity_stores[Commodity::Food],
            self.commodity_stores[Commodity::Lumber],
            self.commodity_stores[Commodity::Luxuries],
            self.commodity_stores[Commodity::Ore],
            self.commodity_stores[Commodity::Stone],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::assert;

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

        k1.unrest = 3;
        k2.unrest = 4;

        k1.commodity_stores[Commodity::Food] = 2;
        k2.commodity_stores[Commodity::Food] = 1;

        k1.commodity_stores[Commodity::Ore] = 5;

        k2.commodity_stores[Commodity::Stone] = 7;

        let diff = k1.diff(&k2);
        assert!(
            diff == vec![
                "Unrest increased from 3 to 4",
                "Food decreased from 2 to 1",
                "Ore decreased from 5 to 0",
                "Stone increased from 0 to 7",
            ]
        );
    }
}
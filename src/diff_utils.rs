use std::{fmt::Display, collections::HashSet};

use crate::Markdownable;

pub fn append_number_change<T: Display + PartialOrd>(diffs: &mut Vec<String>, aspect: &str, first: T, second: T) {
    if first < second {
        diffs.push(format!("{aspect} increased from {first} to {second}"));
    }
    else if first > second {
        diffs.push(format!("{aspect} decreased from {first} to {second}"));
    }
}

pub fn append_bool_change(
    diffs: &mut Vec<String>,
    first: bool, lost_desc: &str,
    second: bool, gained_desc: &str,
) {
    if first && !second {
        diffs.push(lost_desc.to_string());
    }
    else if !first && second {
        diffs.push(gained_desc.to_string());
    }
}

pub fn append_set_change<T: Markdownable>(
    diffs: &mut Vec<String>,
    aspect: &str,
    first: &Vec<T>,
    second: &Vec<T>,
) {
    let first_set: HashSet<_> = first.iter()
        .map(|item| item.to_markdown())
        .collect();

    let second_set: HashSet<_> = second.iter()
        .map(|item| item.to_markdown())
        .collect();

    for item in first_set.difference(&second_set) {
        diffs.push(format!("Lost {aspect}: {item}"));
    }

    for item in second_set.difference(&first_set) {
        diffs.push(format!("Gained {aspect}: {item}"));
    }
}

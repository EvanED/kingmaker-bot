use std::fmt::Display;

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

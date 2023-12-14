#[derive(Debug)]
pub struct DieRoll {
    pub natural: NaturalRoll,
    pub total: TotalRoll,
    pub description: String,
}

impl DieRoll {
    pub fn to_markdown(&self) -> String {
        format!(
            "**{}** total = {}",
            self.total.0,
            self.description,
        )
    }
}

#[derive(Debug)]
pub struct RollResult {
    pub die_roll: DieRoll,
    pub degree: DegreeOfSuccess,
    pub dc: DC,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DegreeOfSuccess {
    CriticalSuccess,
    Success,
    Failure,
    CriticalFailure,
}

impl DegreeOfSuccess {
    pub fn to_markdown(self) -> &'static str {
        use DegreeOfSuccess::*;
        match self {
            CriticalSuccess => "Critical Success",
            Success         => "Success",
            Failure         => "Failure",
            CriticalFailure => "Critical Failure",
        }
    }

    pub fn lowercase_description(self) -> &'static str {
        use DegreeOfSuccess::*;
        match self {
            CriticalSuccess => "critical success",
            Success         => "success",
            Failure         => "failure",
            CriticalFailure => "critical failure",
        }
    }

    pub fn improve(&self) -> DegreeOfSuccess {
        use DegreeOfSuccess::*;
        match self {
            CriticalSuccess => CriticalSuccess,
            Success         => CriticalSuccess,
            Failure         => Success,
            CriticalFailure => Failure,
        }
    }

    pub fn degrade(&self) -> DegreeOfSuccess {
        use DegreeOfSuccess::*;
        match self {
            CriticalSuccess => Success,
            Success         => Failure,
            Failure         => CriticalFailure,
            CriticalFailure => CriticalFailure,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DC(pub i8);

#[derive(Debug, Clone, Copy)]
pub struct NaturalRoll(pub i8);

#[derive(Debug, Clone, Copy)]
pub struct TotalRoll(pub i8);

fn rate_success_no_nat_1_or_20(total: TotalRoll, dc: DC) -> DegreeOfSuccess {
    if total.0 >= dc.0 + 10 {
        DegreeOfSuccess::CriticalSuccess
    }
    else if total.0 >= dc.0 {
        DegreeOfSuccess::Success
    }
    else if total.0 >= dc.0 - 10 {
        DegreeOfSuccess::Failure
    }
    else {
        DegreeOfSuccess::CriticalFailure
    }
}

pub fn rate_success(natural: NaturalRoll, total: TotalRoll, dc: DC) -> DegreeOfSuccess {
    let initial_degree = rate_success_no_nat_1_or_20(total, dc);
    match natural.0 {
        1  => initial_degree.degrade(),
        20 => initial_degree.improve(),
        _  => initial_degree,
    }
}


#[cfg(test)]
mod tests {
    use assert2::assert;
    use super::*;

    #[test]
    fn if_it_meets_it_beats() {
        assert!(rate_success(NaturalRoll(5), TotalRoll(15), DC(15)) == DegreeOfSuccess::Success);
    }

    #[test]
    fn dc_plus_9_still_only_success() {
        assert!(rate_success(NaturalRoll(5), TotalRoll(24), DC(15)) == DegreeOfSuccess::Success);
    }

    #[test]
    fn dc_plus_10_is_crit() {
        assert!(rate_success(NaturalRoll(5), TotalRoll(25), DC(15)) == DegreeOfSuccess::CriticalSuccess);
    }

    #[test]
    fn dc_minus_1_is_failure() {
        assert!(rate_success(NaturalRoll(5), TotalRoll(14), DC(15)) == DegreeOfSuccess::Failure);
    }

    #[test]
    fn dc_minus_9_is_failure() {
        assert!(rate_success(NaturalRoll(5), TotalRoll(6), DC(15)) == DegreeOfSuccess::Failure);
    }

    #[test]
    fn dc_minus_10_is_failure_under_my_house_rule() {
        assert!(rate_success(NaturalRoll(5), TotalRoll(5), DC(15)) == DegreeOfSuccess::Failure);
    }

    #[test]
    fn dc_minus_11_is_critical_failure() {
        assert!(rate_success(NaturalRoll(5), TotalRoll(4), DC(15)) == DegreeOfSuccess::CriticalFailure);
    }

    #[test]
    fn nat_20_raises_critical_failure_to_failure() {
        assert!(rate_success(NaturalRoll(20), TotalRoll(-100), DC(15)) == DegreeOfSuccess::Failure);
    }

    #[test]
    fn nat_20_raises_success_to_critical_success() {
        assert!(rate_success(NaturalRoll(20), TotalRoll(15), DC(15)) == DegreeOfSuccess::CriticalSuccess);
    }

    #[test]
    fn nat_20_raises_critical_success_to_critical_success() {
        assert!(rate_success(NaturalRoll(20), TotalRoll(30), DC(15)) == DegreeOfSuccess::CriticalSuccess);
    }

    #[test]
    fn nat_1_lowers_critical_success_to_success() {
        assert!(rate_success(NaturalRoll(1), TotalRoll(100), DC(15)) == DegreeOfSuccess::Success);
    }

    #[test]
    fn nat_1_lowers_failure_to_critical_failure() {
        assert!(rate_success(NaturalRoll(1), TotalRoll(10), DC(15)) == DegreeOfSuccess::CriticalFailure);
    }

    #[test]
    fn nat_1_lowers_critical_failure_to_critical_failure() {
        assert!(rate_success(NaturalRoll(1), TotalRoll(-100), DC(15)) == DegreeOfSuccess::CriticalFailure);
    }
}
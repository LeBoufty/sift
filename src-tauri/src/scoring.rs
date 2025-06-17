use crate::models::{CategoryResponse, CheckResponse, CheckType, ChecklistResponse, ScoringRule};

impl ScoringRule {
    /** Calculates the score for a given value following the rule. */
    // Check src/models.rs for specifics on each rule.
    pub fn compute_score(&self, value: &f32) -> f32 {
        match self {
            Self::ZeroToOne(copt) => {
                if value > copt {
                    100.0
                } else {
                    0.0
                }
            }
            Self::OneToZero(copt) => {
                if copt > value {
                    100.0
                } else {
                    0.0
                }
            }
            Self::Linear(a, b) => {
                let m: f32 = 100.0 / (b - a);
                let c: f32 = (100.0 * a) / (a - b);
                let score = m * value + c;
                if score >= 100.0 {
                    100.0
                } else if score <= 0.0 {
                    0.0
                } else {
                    score
                }
            }
            Self::Proximity(copt) => {
                let score = 100.0 - (value - copt).abs() / copt;
                if score <= 0.0 {
                    0.0
                } else {
                    score
                }
            }
            Self::Distance(copt) => {
                let score = (value - copt).abs() / copt;
                if score >= 100.0 {
                    100.0
                } else {
                    score
                }
            }
        }
    }
}

impl CheckResponse {
    pub fn get_score(&self) -> f32 {
        match &self.check.ctype {
            CheckType::Checkbox => self.value,
            CheckType::Stars => self.value,
            CheckType::Value(rule) => rule.compute_score(&self.value),
        }
    }
}

impl CategoryResponse {
    pub fn get_total_weight(&self) -> f32 {
        self.checks.iter().map(|r| r.check.weight).sum()
    }

    pub fn get_score(&self) -> f32 {
        let tweight = self.get_total_weight();
        self.checks
            .iter()
            .map(|r| r.get_score() * r.check.weight / tweight)
            .sum()
    }
}

impl ChecklistResponse {
    pub fn get_score(self) -> f32 {
        self.categories.iter().map(|c| c.get_score()).sum::<f32>() / (self.categories.len() as f32)
    }
}

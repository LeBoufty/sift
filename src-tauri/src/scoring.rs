use crate::models::ScoringRule;

impl ScoringRule {
    /** Calculates the score for a given value following the rule. */
    // Check src/models.rs for specifics on each rule.
    pub fn compute_score(self, value: f32) -> f32 {
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

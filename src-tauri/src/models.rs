use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/** The rule by which the score is calculated */
pub enum ScoringRule {
    /** Curve goes from 0 to 100 at cutoff point */
    ZeroToOne(f32),
    /** Curve goes from 100 to 0 at cutoff point */
    OneToZero(f32),
    /** Score increases the closest it is to desired value */
    Proximity(f32),
    /** Score increases the furthest it is to undesired value */
    Distance(f32),
    /** Curve is 0 until first value and 100 from second value */
    Linear(f32, f32),
}

#[derive(Serialize, Deserialize)]
/** The type of a check in a checklist */
pub enum CheckType {
    /** Simple checkbox */
    Checkbox,
    /** 5-stars rating */
    Stars,
    /** Number input with custom scoring */
    Value(ScoringRule),
}

#[derive(Serialize, Deserialize)]
/** A check in a checklist */
pub struct Check {
    /** Check name */
    pub name: String,
    /** Check type */
    pub ctype: CheckType,
    /** Check weight */
    pub weight: f32,
}

use serde::{Deserialize, Serialize};

// Template models
// Used to construct the form in the frontend

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

#[derive(Serialize, Deserialize)]
/** A category in a checklist */
pub struct ChecklistCategory {
    /** Category name */
    pub name: String,
    /** Checks in the category */
    pub checks: Vec<Check>,
}

#[derive(Serialize, Deserialize)]
/** A checklist template, which is used to rank entries */
pub struct Checklist {
    /** Checklist ID */
    pub id: usize,
    /** Checklist name */
    pub name: String,
    /** Categories in the checklist */
    pub categories: Vec<ChecklistCategory>,
}

// Response models
// Used for score computations

#[derive(Serialize, Deserialize)]
pub struct CheckResponse {
    /** The check that is being responded to */
    pub check: Check,
    /** The value given by the user */
    pub value: f32,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryResponse {
    /** Name of the category, same as in template */
    pub name: String,
    /** Individual checks' responses */
    pub checks: Vec<CheckResponse>,
}

#[derive(Serialize, Deserialize)]
pub struct ChecklistResponse {
    /** Template ID */
    pub template_id: usize,
    /** Response name, different from checklist name */
    pub name: String,
    /** Categories in the response */
    pub categories: Vec<CategoryResponse>,
}

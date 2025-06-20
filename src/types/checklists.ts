/** Interface for the buttons in lists */
export interface ChecklistInfo {
  id: number;
  name: string;
  entryCount: number;
}

// Types from models.rs
export const ZEROTOONE = "ZeroToOne";
export const ONETOZERO = "OneToZero";
export const PROXIMITY = "Proximity";
export const DISTANCE = "Distance";
export const LINEAR = "Linear";
export type ScoringRuleType =
  | typeof ZEROTOONE
  | typeof ONETOZERO
  | typeof PROXIMITY
  | typeof DISTANCE
  | typeof LINEAR;

export const CHECKBOX = "Checkbox";
export const STARS = "Stars";
export const VALUE = "Value";
export type CheckTypeType =
  | typeof CHECKBOX
  | typeof STARS
  | typeof VALUE;

export interface ScoringRule {
  type: ScoringRuleType;
  value: number | number[];
}

export interface CheckType {
  type: CheckTypeType;
  value?: ScoringRule;
}

export interface Check {
  name: string;
  ctype: CheckType;
  weight: number;
}

export interface ChecklistCategory {
  id: number;
  name: string;
  checks: Check[];
}

export interface Checklist {
  id: number;
  name: string;
  categories: ChecklistCategory[];
}

export function default_check(n?: number): Check {
  return {
    name: `Check #${n ?? 1}`,
    ctype: {
      type: "Stars",
    },
    weight: 1
  }
}

export function default_category(id?: number): ChecklistCategory {
  return {
    id: id ?? 0,
    name: `Category #${id ?? 0 + 1}`,
    checks: [
      default_check()
    ],
  }
}

export function default_checklist(id?: number): Checklist {
  return {
    id: id ?? 0,
    name: `Checklist #${id ?? 0 + 1}`,
    categories: [
      default_category()
    ]
  }
}

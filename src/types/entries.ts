import type { Check } from "./checklists";

export interface CheckResponse {
  check: Check;
  value: number;
}

export interface CategoryResponse {
  name: string;
  checks: CheckResponse[];
}

export interface ChecklistResponse {
  template_id: number;
  name: string;
  categories: CategoryResponse[];
}

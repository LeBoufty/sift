// src/routes/settings/settings.ts
import { writable } from "svelte/store";

export const GRADING_FORMATS = ["Percentage", "/20", "Letters", "5-stars"];

export const SETTINGS = writable({
  theme: "light",
  // Grading format : Percentage, /20, Letters, 5-stars
  grades: "Percentage"
});

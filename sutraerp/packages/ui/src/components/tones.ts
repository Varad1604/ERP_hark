/** Pastel tint families of the Soft Bento palette. */
export type Tone = "orange" | "green" | "violet" | "blue" | "rose" | "amber" | "grey" | "teal";

export const TONES: readonly Tone[] = ["orange", "green", "violet", "blue", "rose", "amber", "grey", "teal"];

export function toneStyle(tone: Tone): { background: string; color: string } {
  return { background: `var(--tint-${tone}-bg)`, color: `var(--tint-${tone}-fg)` };
}

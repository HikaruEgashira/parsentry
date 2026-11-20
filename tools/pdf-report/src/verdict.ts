import type { SarifResult } from "./types.js";

/** A result is indeterminate unless its verdict is explicitly `confirmed` or `ruled_out`. */
export function isIndeterminate(r: SarifResult): boolean {
  const v = r.properties?.verdict;
  return v !== "confirmed" && v !== "ruled_out";
}

/**
 * Split results the way report.md does: confirmed (and ruled-out) findings form
 * the main body, indeterminate findings go under an "Unverified" section.
 * Order within each group is preserved.
 */
export function groupByVerdict(results: SarifResult[]): {
  verified: SarifResult[];
  unverified: SarifResult[];
} {
  const verified: SarifResult[] = [];
  const unverified: SarifResult[] = [];
  for (const r of results) (isIndeterminate(r) ? unverified : verified).push(r);
  return { verified, unverified };
}

import { test } from "node:test";
import assert from "node:assert/strict";
import React from "react";
import { renderToBuffer } from "@react-pdf/renderer";
import { groupByVerdict } from "../src/verdict.js";
import { FindingPages } from "../src/components/FindingPage.js";
import { Document } from "@react-pdf/renderer";
import type { SarifResult } from "../src/types.js";

const result = (ruleId: string, verdict?: string): SarifResult => ({
  ruleId,
  level: "error",
  message: { text: ruleId },
  locations: [],
  properties: verdict ? { verdict } : {},
});

const fixture = [
  result("UNVERIFIED-A", "indeterminate"),
  result("CONFIRMED-A", "confirmed"),
  result("UNVERIFIED-B"),
  result("RULED-OUT-A", "ruled_out"),
];

test("confirmed and ruled-out form the main body, indeterminate is unverified", () => {
  const { verified, unverified } = groupByVerdict(fixture);
  assert.deepEqual(verified.map((r) => r.ruleId), ["CONFIRMED-A", "RULED-OUT-A"]);
  assert.deepEqual(unverified.map((r) => r.ruleId), ["UNVERIFIED-A", "UNVERIFIED-B"]);
});

test("PDF renders with both groups", async () => {
  const buf = await renderToBuffer(
    <Document>
      <FindingPages results={fixture} rules={[]} />
    </Document>
  );
  assert.ok(buf.length > 0);
});

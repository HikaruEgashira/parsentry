import React from "react";
import { renderToFile } from "@react-pdf/renderer";
import { readFileSync, existsSync, statSync } from "node:fs";
import { resolve, basename, dirname, join } from "node:path";
import { Report } from "./components/Report.js";
import { parseReportMarkdown, type MarkdownReport } from "./markdown-parser.js";
import type { SarifReport, ReportData } from "./types.js";

function usage(): never {
  console.error("Usage: tsx src/index.tsx <report-dir|sarif-file> [output.pdf]");
  console.error("");
  console.error("  report-dir  : directory containing merged.sarif.json and report.md");
  console.error("  sarif-file  : path to a .sarif.json file (report.md auto-detected)");
  console.error("  output.pdf  : optional output path (default: report.pdf)");
  process.exit(1);
}

function parseSarif(path: string): SarifReport {
  if (!existsSync(path)) {
    console.error(`File not found: ${path}`);
    process.exit(1);
  }
  return JSON.parse(readFileSync(path, "utf-8"));
}

function buildReportData(sarif: SarifReport, inputPath: string): ReportData {
  const run = sarif.runs[0];
  if (!run) {
    console.error("No runs found in SARIF file");
    process.exit(1);
  }

  const results = run.results.filter((r) => r.baselineState !== "absent");

  let critical = 0,
    high = 0,
    medium = 0,
    low = 0,
    info = 0;

  for (const r of results) {
    const conf = r.properties?.confidence;
    const score = conf !== undefined ? (conf <= 1 ? conf * 100 : conf) : undefined;

    if (r.level === "error") {
      if (score !== undefined && score >= 90) critical++;
      else high++;
    } else if (r.level === "warning") {
      medium++;
    } else if (r.level === "note") {
      low++;
    } else {
      info++;
    }
  }

  const dir = basename(resolve(inputPath, ".."));
  const title = dir !== "." ? dir : basename(inputPath, ".sarif.json");

  return {
    title,
    generatedAt: new Date().toISOString().split("T")[0],
    toolName: run.tool.driver.name,
    toolVersion: run.tool.driver.version,
    results,
    rules: run.tool.driver.rules ?? [],
    stats: { total: results.length, critical, high, medium, low, info },
  };
}

function resolvePaths(input: string): { sarifPath: string; reportMdPath: string | null; outputDefault: string } {
  const resolved = resolve(input);

  // If input is a directory, look for known files inside
  if (existsSync(resolved) && statSync(resolved).isDirectory()) {
    const sarifPath = join(resolved, "merged.sarif.json");
    const reportMdPath = join(resolved, "report.md");
    return {
      sarifPath,
      reportMdPath: existsSync(reportMdPath) ? reportMdPath : null,
      outputDefault: join(resolved, "report.pdf"),
    };
  }

  // Input is a sarif file — look for report.md in the same directory
  const dir = dirname(resolved);
  const reportMdPath = join(dir, "report.md");
  return {
    sarifPath: resolved,
    reportMdPath: existsSync(reportMdPath) ? reportMdPath : null,
    outputDefault: join(dir, "report.pdf"),
  };
}

async function main() {
  const args = process.argv.slice(2);
  if (args.length < 1) usage();

  const { sarifPath, reportMdPath, outputDefault } = resolvePaths(args[0]);
  const outputPath = resolve(args[1] ?? outputDefault);

  console.log(`SARIF: ${sarifPath}`);
  const sarif = parseSarif(sarifPath);
  const data = buildReportData(sarif, sarifPath);

  let markdown: MarkdownReport | undefined;
  if (reportMdPath) {
    console.log(`Report MD: ${reportMdPath}`);
    markdown = parseReportMarkdown(reportMdPath);
    // Override title from markdown if available
    if (markdown.title) data.title = markdown.title;
    if (markdown.date) data.generatedAt = markdown.date;
    console.log(
      `  Categories: ${markdown.categories.length}, Remediation groups: ${markdown.remediationGroups.length}, Findings: ${markdown.findings.length}`
    );
  } else {
    console.log(`Report MD: not found (SARIF-only mode)`);
  }

  console.log(
    `Findings: ${data.stats.total} total (${data.stats.critical} critical, ${data.stats.high} high, ${data.stats.medium} medium, ${data.stats.low} low, ${data.stats.info} info)`
  );

  console.log(`Rendering PDF...`);
  await renderToFile(<Report data={data} markdown={markdown} />, outputPath);
  console.log(`Saved: ${outputPath}`);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});

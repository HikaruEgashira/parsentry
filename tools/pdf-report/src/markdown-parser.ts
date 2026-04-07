import { readFileSync } from "node:fs";

export interface VulnCategory {
  category: string;
  count: number;
  impact: string;
}

export interface RemediationGroup {
  priority: string; // "Immediate (Critical Risk)" etc.
  items: string[];
}

export interface FindingDetail {
  index: number;
  title: string;
  ruleId: string;
  severity: string;
  confidence: string;
  location: string;
  description: string;
  remediation: string;
}

export interface MarkdownReport {
  title: string;
  date: string;
  target: string;
  surfacesAnalyzed: string;
  categories: VulnCategory[];
  remediationGroups: RemediationGroup[];
  findings: FindingDetail[];
}

export function parseReportMarkdown(path: string): MarkdownReport {
  const content = readFileSync(path, "utf-8");
  const lines = content.split("\n");

  // Title
  const titleMatch = lines[0]?.match(/^#\s+Security Analysis Report:\s*(.+)/);
  const title = titleMatch?.[1] ?? "Unknown";

  // Metadata
  const date = extract(lines, /^\*\*Date:\*\*\s*(.+)/);
  const target = extract(lines, /^\*\*Target:\*\*\s*(.+)/);
  const surfacesAnalyzed = extract(lines, /^\*\*Surfaces Analyzed:\*\*\s*(.+)/);

  // Top Vulnerability Categories table
  const categories = parseCategoriesTable(content);

  // Remediation Priority
  const remediationGroups = parseRemediationPriority(content);

  // Findings
  const findings = parseFindings(content);

  return { title, date, target, surfacesAnalyzed, categories, remediationGroups, findings };
}

function extract(lines: string[], regex: RegExp): string {
  for (const line of lines) {
    const m = line.match(regex);
    if (m) return m[1].trim();
  }
  return "";
}

function parseCategoriesTable(content: string): VulnCategory[] {
  const section = extractSection(content, "### Top Vulnerability Categories", "\n---\n");
  if (!section) return [];

  const results: VulnCategory[] = [];
  const tableLines = section.split("\n").filter((l) => l.startsWith("|") && !l.includes("---"));

  for (const line of tableLines.slice(1)) {
    // skip header
    const cols = line
      .split("|")
      .map((c) => c.trim())
      .filter(Boolean);
    if (cols.length >= 3) {
      results.push({
        category: cols[0],
        count: parseInt(cols[1], 10) || 0,
        impact: cols[2],
      });
    }
  }
  return results;
}

function parseRemediationPriority(content: string): RemediationGroup[] {
  const section = extractSection(content, "## Remediation Priority", null);
  if (!section) return [];

  const groups: RemediationGroup[] = [];
  let current: RemediationGroup | null = null;

  for (const line of section.split("\n")) {
    const headerMatch = line.match(/^###\s+(.+)/);
    if (headerMatch) {
      if (current) groups.push(current);
      current = { priority: headerMatch[1].trim(), items: [] };
      continue;
    }
    if (current && line.match(/^\d+\.\s+/)) {
      const item = line.replace(/^\d+\.\s+/, "").trim();
      current.items.push(item);
    }
  }
  if (current) groups.push(current);
  return groups;
}

function parseFindings(content: string): FindingDetail[] {
  const section = extractSection(content, "## Findings Detail", "## Remediation Priority");
  if (!section) return [];

  const findings: FindingDetail[] = [];
  const blocks = section.split(/\n(?=###\s+\d)/);

  for (const block of blocks) {
    const headerMatch = block.match(/###\s+(\d+[\-\d]*)\.\s+(.+)/);
    if (!headerMatch) continue;

    const index = parseInt(headerMatch[1], 10);
    const title = headerMatch[2].trim();
    const ruleId = extractField(block, /\*\*Rule ID:\*\*\s*(.+)/);
    const sevConf = extractField(block, /\*\*Severity:\*\*\s*(.+)/);
    const severity = sevConf.split("|")[0]?.trim() ?? "";
    const confidence = sevConf.split("Confidence:**")[1]?.trim() ?? "";
    const location = extractField(block, /\*\*Locations?:\*\*\s*(.+)/);
    const description = extractField(block, /\*\*Description:\*\*\s*(.+)/);
    const remediation = extractField(block, /\*\*Remediation:\*\*\s*(.+)/);

    findings.push({ index, title, ruleId, severity, confidence, location, description, remediation });
  }
  return findings;
}

function extractField(text: string, regex: RegExp): string {
  const m = text.match(regex);
  return m?.[1]?.trim() ?? "";
}

function extractSection(content: string, startMarker: string, endMarker: string | null): string | null {
  const startIdx = content.indexOf(startMarker);
  if (startIdx === -1) return null;

  const start = startIdx + startMarker.length;
  if (endMarker) {
    const endIdx = content.indexOf(endMarker, start);
    return endIdx === -1 ? content.slice(start) : content.slice(start, endIdx);
  }
  return content.slice(start);
}

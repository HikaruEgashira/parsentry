// Read-only tools exposed to the agent. Every path is confined to the scan root.

import { existsSync, readdirSync, readFileSync, realpathSync, statSync } from "node:fs";
import path from "node:path";

const MAX_FILE_BYTES = 256 * 1024;
const MAX_GREP_MATCHES = 200;
const SKIP_DIRS = new Set([".git", "node_modules", "target", "dist", "vendor", ".venv"]);

/** Resolve `rel` under `root`, rejecting paths (or symlinks) that escape it. */
export function resolveInside(root: string, rel: string): string {
  const base = realpathSync(root);
  const full = path.resolve(base, rel);
  const real = existsSync(full) ? realpathSync(full) : full;
  if (real !== base && !real.startsWith(base + path.sep)) {
    throw new Error(`path escapes the scan root: ${rel}`);
  }
  return real;
}

export function readFile(root: string, file: string, offset = 1, limit = 2000): string {
  const full = resolveInside(root, file);
  if (statSync(full).size > MAX_FILE_BYTES * 8) throw new Error(`file too large: ${file}`);
  const lines = readFileSync(full, "utf8").split("\n");
  const start = Math.max(offset, 1) - 1;
  return lines
    .slice(start, start + limit)
    .map((line, i) => `${start + i + 1}\t${line}`)
    .join("\n")
    .slice(0, MAX_FILE_BYTES);
}

export function grep(root: string, pattern: string, dir = ".", glob?: string): string {
  const regex = new RegExp(pattern);
  const suffix = glob?.replace(/^\*+/, "");
  const base = realpathSync(root);
  const matches: string[] = [];
  const walk = (current: string) => {
    for (const entry of readdirSync(current, { withFileTypes: true })) {
      if (matches.length >= MAX_GREP_MATCHES) return;
      const full = path.join(current, entry.name);
      if (entry.isDirectory()) {
        if (!SKIP_DIRS.has(entry.name)) walk(full);
      } else if (entry.isFile() && (!suffix || entry.name.endsWith(suffix))) {
        if (statSync(full).size > MAX_FILE_BYTES) continue;
        const lines = readFileSync(full, "utf8").split("\n");
        lines.forEach((line, i) => {
          if (matches.length < MAX_GREP_MATCHES && regex.test(line)) {
            matches.push(`${path.relative(base, full)}:${i + 1}:${line.slice(0, 300)}`);
          }
        });
      }
    }
  };
  walk(resolveInside(root, dir));
  return matches.length ? matches.join("\n") : "no matches";
}

/** Skill directories searched in order: $PARSENTRY_SKILLS_DIR, <root>/.claude/skills, <root>/skills. */
export function loadSkill(root: string, name: string): string {
  if (!/^[A-Za-z0-9_.-]+$/.test(name) || name.includes("..")) {
    throw new Error(`invalid skill name: ${name}`);
  }
  const dirs = [
    process.env.PARSENTRY_SKILLS_DIR,
    path.join(root, ".claude", "skills"),
    path.join(root, "skills"),
  ].filter((d): d is string => Boolean(d));
  for (const dir of dirs) {
    const file = path.join(dir, name, "SKILL.md");
    if (existsSync(file)) return readFileSync(file, "utf8");
  }
  throw new Error(`skill not found: ${name}`);
}

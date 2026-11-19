/** Read-only tools exposed to the agent, confined to the scan root. */
import { promises as fs } from "node:fs";
import path from "node:path";

const MAX_READ_BYTES = 256 * 1024;
const MAX_GREP_MATCHES = 200;
const SKIP_DIRS = new Set([".git", "node_modules", "target"]);
const SKILL_NAME = /^[A-Za-z0-9][A-Za-z0-9_-]*$/;

/**
 * Resolve `rel` inside `root`, rejecting `..` traversal, absolute paths
 * outside the root, and symlinks that point out of it.
 */
export async function resolveInRoot(root: string, rel: string): Promise<string> {
  const realRoot = await fs.realpath(root);
  const real = await fs.realpath(path.resolve(realRoot, rel));
  const relative = path.relative(realRoot, real);
  if (relative.startsWith("..") || path.isAbsolute(relative)) {
    throw new Error(`path escapes scan root: ${rel}`);
  }
  return real;
}

export async function readFile(root: string, rel: string): Promise<string> {
  const file = await resolveInRoot(root, rel);
  const stat = await fs.stat(file);
  if (!stat.isFile()) throw new Error(`not a file: ${rel}`);
  const handle = await fs.open(file, "r");
  try {
    const buf = Buffer.alloc(Math.min(stat.size, MAX_READ_BYTES));
    const { bytesRead } = await handle.read(buf, 0, buf.length, 0);
    const text = buf.subarray(0, bytesRead).toString("utf8");
    return stat.size > MAX_READ_BYTES ? `${text}\n[truncated at ${MAX_READ_BYTES} bytes]` : text;
  } finally {
    await handle.close();
  }
}

export interface GrepMatch {
  path: string;
  line: number;
  text: string;
}

export async function grep(root: string, pattern: string, rel = "."): Promise<GrepMatch[]> {
  const regex = new RegExp(pattern);
  const realRoot = await fs.realpath(root);
  const start = await resolveInRoot(realRoot, rel);
  const matches: GrepMatch[] = [];

  const visit = async (abs: string): Promise<void> => {
    if (matches.length >= MAX_GREP_MATCHES) return;
    const stat = await fs.lstat(abs);
    if (stat.isSymbolicLink()) return;
    if (stat.isDirectory()) {
      if (SKIP_DIRS.has(path.basename(abs)) && abs !== start) return;
      const entries = (await fs.readdir(abs)).sort();
      for (const entry of entries) await visit(path.join(abs, entry));
      return;
    }
    if (!stat.isFile() || stat.size > MAX_READ_BYTES) return;
    const text = await fs.readFile(abs, "utf8");
    if (text.includes("\0")) return;
    const lines = text.split("\n");
    for (let i = 0; i < lines.length && matches.length < MAX_GREP_MATCHES; i++) {
      if (regex.test(lines[i])) {
        matches.push({ path: path.relative(realRoot, abs), line: i + 1, text: lines[i] });
      }
    }
  };

  await visit(start);
  return matches;
}

/** Load `<root>/.claude/skills/<name>/SKILL.md`. */
export async function loadSkill(root: string, name: string): Promise<string> {
  if (!SKILL_NAME.test(name)) throw new Error(`invalid skill name: ${name}`);
  return readFile(root, path.join(".claude", "skills", name, "SKILL.md"));
}

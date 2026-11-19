import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { test } from "node:test";
import { grep, loadSkill, readFile } from "../src/tools.ts";

const base = mkdtempSync(path.join(tmpdir(), "agent-tools-"));
const root = path.join(base, "repo");
mkdirSync(path.join(root, "src"), { recursive: true });
mkdirSync(path.join(root, ".claude", "skills", "audit"), { recursive: true });
writeFileSync(path.join(root, "src", "app.py"), "import os\nos.system(cmd)\n");
writeFileSync(path.join(root, ".claude", "skills", "audit", "SKILL.md"), "# audit");
writeFileSync(path.join(base, "secret.txt"), "outside");
symlinkSync(path.join(base, "secret.txt"), path.join(root, "link.txt"));

test("read_file reads inside the root", async () => {
  assert.equal(await readFile(root, "src/app.py"), "import os\nos.system(cmd)\n");
});

test("read_file rejects traversal, absolute and symlink escapes", async () => {
  await assert.rejects(readFile(root, "../secret.txt"), /escapes scan root/);
  await assert.rejects(readFile(root, path.join(base, "secret.txt")), /escapes scan root/);
  await assert.rejects(readFile(root, "link.txt"), /escapes scan root/);
});

test("grep finds matches and stays in the root", async () => {
  assert.deepEqual(await grep(root, "os\\.system"), [{ path: "src/app.py", line: 2, text: "os.system(cmd)" }]);
  assert.deepEqual(await grep(root, "outside"), []);
  await assert.rejects(grep(root, "x", ".."), /escapes scan root/);
});

test("load_skill loads by name and rejects path names", async () => {
  assert.equal(await loadSkill(root, "audit"), "# audit");
  await assert.rejects(loadSkill(root, "../../x"), /invalid skill name/);
});

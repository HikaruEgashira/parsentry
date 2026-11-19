import assert from "node:assert/strict";
import { mkdtempSync, mkdirSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { test } from "node:test";
import { isRetryable, mapConcurrent, withRetry } from "./limits.js";
import { respond } from "./rpc.js";
import { grep, loadSkill, readFile } from "./tools.js";

const limits = { concurrency: 2, timeoutMs: 1000, maxRetries: 2, baseDelayMs: 1 };

test("run_surface keeps per-task errors and order", async () => {
  const backends = {
    claude: async (prompt: string) => {
      if (prompt === "bad") throw new Error("boom");
      return `ok:${prompt}`;
    },
  };
  const line = JSON.stringify({
    jsonrpc: "2.0",
    id: 7,
    method: "run_surface",
    params: { tasks: [{ id: "a", prompt: "x" }, { id: "b", prompt: "bad" }] },
  });
  const res = JSON.parse(await respond(line, backends, limits));
  assert.equal(res.id, 7);
  assert.deepEqual(res.result.results, [{ id: "a", text: "ok:x" }, { id: "b", error: "boom" }]);
});

test("unknown method is a JSON-RPC error", async () => {
  const res = JSON.parse(await respond('{"jsonrpc":"2.0","id":1,"method":"nope"}', {}, limits));
  assert.equal(res.error.code, -32601);
});

test("429/529 are retried, other errors are not", async () => {
  assert.ok(isRetryable(new Error("API Error: 529 overloaded")));
  assert.ok(!isRetryable(new Error("invalid request")));
  let calls = 0;
  const value = await withRetry(async () => {
    if (++calls < 3) throw new Error("429 rate limit");
    return "done";
  }, limits);
  assert.equal(value, "done");
  assert.equal(calls, 3);
});

test("timeout aborts a hung task", async () => {
  await assert.rejects(
    withRetry(() => new Promise(() => {}), { ...limits, timeoutMs: 10 }),
    /timed out/,
  );
});

test("mapConcurrent respects the limit", async () => {
  let active = 0;
  let peak = 0;
  await mapConcurrent([1, 2, 3, 4, 5], 2, async () => {
    peak = Math.max(peak, ++active);
    await new Promise((r) => setTimeout(r, 5));
    active--;
  });
  assert.equal(peak, 2);
});

test("tools are confined to the scan root", () => {
  const root = mkdtempSync(path.join(tmpdir(), "agent-runtime-"));
  writeFileSync(path.join(root, "app.py"), "import os\nos.system(cmd)\n");
  mkdirSync(path.join(root, "skills", "sec"), { recursive: true });
  writeFileSync(path.join(root, "skills", "sec", "SKILL.md"), "# sec");
  assert.equal(readFile(root, "app.py", 2, 1), "2\tos.system(cmd)");
  assert.equal(grep(root, "system"), "app.py:2:os.system(cmd)");
  assert.equal(loadSkill(root, "sec"), "# sec");
  assert.throws(() => readFile(root, "../etc/passwd"), /escapes/);
  assert.throws(() => loadSkill(root, "../x"), /invalid/);
});

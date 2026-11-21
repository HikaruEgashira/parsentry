/** JSON-RPC contract tests shared by every backend; SDK calls are stubbed, so no network is used. */
import assert from "node:assert/strict";
import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { describe, test } from "node:test";
import { claudeBackend, type ClaudeDeps } from "../src/claude.ts";
import { type Config, loadConfig } from "../src/config.ts";
import { openaiBackend, type OpenAIDeps } from "../src/openai.ts";
import { INVALID_PARAMS } from "../src/rpc.ts";
import { type Backend, createDispatcher } from "../src/runtime.ts";

const root = mkdtempSync(path.join(tmpdir(), "agent-runtime-"));
const config = loadConfig({ PARSENTRY_AGENT_TIMEOUT_MS: "50", PARSENTRY_AGENT_MAX_RETRIES: "2" });

/** Produces the agent's final text for a prompt, or throws to simulate an API failure. */
type Respond = (prompt: string) => string;

function rateLimited(): Error {
  return Object.assign(new Error("rate limited"), { status: 429 });
}

const backends: Array<{ name: string; make: (config: Config, respond: Respond) => Backend }> = [
  {
    name: "claude",
    make: (config, respond) =>
      claudeBackend(config, {
        query: (({ prompt }: { prompt: string }) =>
          (async function* () {
            yield { type: "result", subtype: "success", result: respond(prompt) };
          })()) as unknown as ClaudeDeps["query"],
      }),
  },
  {
    name: "openai",
    make: (config, respond) =>
      openaiBackend(config, {
        run: (async (_agent: unknown, input: string) => ({
          finalOutput: respond(input),
        })) as unknown as OpenAIDeps["run"],
      }),
  },
];

for (const { name, make } of backends) {
  describe(`contract: ${name} backend`, () => {
    test("run retries the backend and reports attempts", async () => {
      let calls = 0;
      const backend = make(config, (prompt) => {
        if (++calls === 1) throw rateLimited();
        return `done: ${prompt}`;
      });
      const d = createDispatcher({ config, backend, sleep: async () => {} });
      const res = await d.handle({ jsonrpc: "2.0", id: 1, method: "run", params: { prompt: "hi", root } });
      assert.deepEqual(res?.result, { output: "done: hi", attempts: 2 });
    });

    test("run surfaces non-retryable failures as errors", async () => {
      const backend = make(config, () => {
        throw new Error("boom");
      });
      const d = createDispatcher({ config, backend, sleep: async () => {} });
      const res = await d.handle({ jsonrpc: "2.0", id: 1, method: "run", params: { prompt: "hi", root } });
      assert.match(res?.error?.message ?? "", /boom/);
    });

    test("run validates params", async () => {
      const d = createDispatcher({ config, backend: make(config, () => "") });
      const res = await d.handle({ jsonrpc: "2.0", id: 1, method: "run", params: { prompt: "hi", root: "rel" } });
      assert.equal(res?.error?.code, INVALID_PARAMS);
    });

    test("initialize lists only the confined tools", async () => {
      const d = createDispatcher({ config, backend: make(config, () => "") });
      const res = await d.handle({ jsonrpc: "2.0", id: 1, method: "initialize" });
      assert.deepEqual((res?.result as { tools: string[] }).tools, ["read_file", "grep", "load_skill"]);
    });
  });
}

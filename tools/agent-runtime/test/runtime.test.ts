import assert from "node:assert/strict";
import { test } from "node:test";
import { loadConfig } from "../src/config.ts";
import { backoffDelay, isRetryable, Semaphore, TimeoutError, withRetry, withTimeout } from "../src/retry.ts";

function rateLimited(): Error {
  return Object.assign(new Error("rate limited"), { status: 429 });
}

test("loadConfig reads PARSENTRY_AGENT_* and validates", () => {
  const c = loadConfig({ PARSENTRY_AGENT_CONCURRENCY: "8", PARSENTRY_AGENT_MODEL: "claude-opus-5-5" });
  assert.equal(c.concurrency, 8);
  assert.equal(c.model, "claude-opus-5-5");
  assert.equal(loadConfig({}).concurrency, 3);
  assert.throws(() => loadConfig({ PARSENTRY_AGENT_CONCURRENCY: "0" }), /PARSENTRY_AGENT_CONCURRENCY/);
  assert.equal(loadConfig({}).backend, "claude");
  assert.equal(loadConfig({ PARSENTRY_AGENT_BACKEND: "openai" }).backend, "openai");
  assert.throws(() => loadConfig({ PARSENTRY_AGENT_BACKEND: "gpt" }), /PARSENTRY_AGENT_BACKEND/);
});

test("isRetryable matches 429/529 only", () => {
  assert.ok(isRetryable({ status: 429 }));
  assert.ok(isRetryable({ status: 529 }));
  assert.ok(isRetryable(new Error("API Error: 529 overloaded_error")));
  assert.ok(!isRetryable({ status: 500 }));
  assert.ok(!isRetryable(new Error("boom")));
});

test("backoff is exponential and capped", () => {
  assert.deepEqual([0, 1, 2, 3].map((a) => backoffDelay(a, 100, 500)), [100, 200, 400, 500]);
});

test("withRetry backs off on 429 then succeeds", async () => {
  const delays: number[] = [];
  let calls = 0;
  const out = await withRetry(
    { maxRetries: 3, backoffBaseMs: 10, backoffMaxMs: 1000, sleep: async (ms) => void delays.push(ms) },
    async () => {
      if (++calls < 3) throw rateLimited();
      return "ok";
    },
  );
  assert.equal(out, "ok");
  assert.deepEqual(delays, [10, 20]);
});

test("withRetry does not retry other errors", async () => {
  let calls = 0;
  await assert.rejects(
    withRetry({ maxRetries: 3, backoffBaseMs: 0, backoffMaxMs: 0 }, async () => {
      calls++;
      throw new Error("boom");
    }),
  );
  assert.equal(calls, 1);
});

test("withTimeout aborts the attempt", async () => {
  let aborted = false;
  await assert.rejects(
    withTimeout(10, (signal) => new Promise(() => signal.addEventListener("abort", () => (aborted = true)))),
    TimeoutError,
  );
  assert.ok(aborted);
});

test("Semaphore limits concurrency", async () => {
  const sem = new Semaphore(2);
  let active = 0;
  let peak = 0;
  await Promise.all(
    Array.from({ length: 6 }, () =>
      sem.run(async () => {
        peak = Math.max(peak, ++active);
        await new Promise((r) => setTimeout(r, 5));
        active--;
      }),
    ),
  );
  assert.equal(peak, 2);
});

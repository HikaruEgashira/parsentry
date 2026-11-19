import assert from "node:assert/strict";
import { test } from "node:test";
import { Dispatcher, INVALID_REQUEST, METHOD_NOT_FOUND, PARSE_ERROR } from "../src/rpc.ts";

const dispatcher = new Dispatcher().register("echo", async (params) => params);

test("unknown method returns a JSON-RPC error", async () => {
  const res = await dispatcher.handleLine(JSON.stringify({ jsonrpc: "2.0", id: 7, method: "nope" }));
  assert.deepEqual(res, { jsonrpc: "2.0", id: 7, error: { code: METHOD_NOT_FOUND, message: "method not found: nope" } });
});

test("dispatches a known method", async () => {
  const res = await dispatcher.handleLine(JSON.stringify({ jsonrpc: "2.0", id: "a", method: "echo", params: [1] }));
  assert.deepEqual(res, { jsonrpc: "2.0", id: "a", result: [1] });
});

test("malformed JSON is a parse error", async () => {
  const res = await dispatcher.handleLine("{");
  assert.equal(res?.error?.code, PARSE_ERROR);
  assert.equal(res?.id, null);
});

test("missing jsonrpc version is an invalid request", async () => {
  const res = await dispatcher.handleLine(JSON.stringify({ id: 1, method: "echo" }));
  assert.equal(res?.error?.code, INVALID_REQUEST);
});

test("notifications get no response", async () => {
  assert.equal(await dispatcher.handleLine(JSON.stringify({ jsonrpc: "2.0", method: "nope" })), undefined);
});

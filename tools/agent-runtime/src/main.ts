#!/usr/bin/env -S npx tsx
/** Sidecar entry point: JSON-RPC requests on stdin, responses on stdout, one per line. */
import readline from "node:readline";
import { claudeBackend } from "./claude.ts";
import { loadConfig } from "./config.ts";
import { openaiBackend } from "./openai.ts";
import { createDispatcher } from "./runtime.ts";

const config = loadConfig();
const rl = readline.createInterface({ input: process.stdin, crlfDelay: Infinity });
const dispatcher = createDispatcher({
  config,
  backend: config.backend === "openai" ? openaiBackend(config) : claudeBackend(config),
  onShutdown: () => rl.close(),
});

rl.on("line", (line) => {
  if (line.trim() === "") return;
  // Requests are handled concurrently; responses are written as they finish.
  void dispatcher.handleLine(line).then((response) => {
    if (response) process.stdout.write(`${JSON.stringify(response)}\n`);
  });
});

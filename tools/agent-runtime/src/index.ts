#!/usr/bin/env node
// parsentry agent runtime sidecar: JSON-RPC 2.0 over stdio, one message per line.
// stdout carries only responses; logs go to stderr.

import { createInterface } from "node:readline";
import { runClaude } from "./claude.js";
import { limitsFromEnv } from "./limits.js";
import { respond } from "./rpc.js";

const backends = { claude: runClaude };
const limits = limitsFromEnv();

const lines = createInterface({ input: process.stdin });
for await (const line of lines) {
  if (!line.trim()) continue;
  process.stdout.write((await respond(line, backends, limits)) + "\n");
}

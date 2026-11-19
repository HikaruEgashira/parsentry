/** Registers the AgentRuntime JSON-RPC methods on a dispatcher. */
import { promises as fs } from "node:fs";
import path from "node:path";
import type { Config } from "./config.ts";
import { Dispatcher, INVALID_PARAMS, RpcError } from "./rpc.ts";
import { Semaphore, withRetry, withTimeout } from "./retry.ts";

export const TOOL_NAMES = ["read_file", "grep", "load_skill"] as const;

export interface RunRequest {
  prompt: string;
  /** Absolute scan root; all tool access is confined to it. */
  root: string;
}

/** A single agent attempt. Must stop work when `signal` aborts. */
export interface Backend {
  name: string;
  run(req: RunRequest, signal: AbortSignal): Promise<string>;
}

export interface RuntimeOptions {
  config: Config;
  backend: Backend;
  sleep?: (ms: number) => Promise<void>;
  onShutdown?: () => void;
}

async function parseRunParams(params: unknown): Promise<RunRequest> {
  if (typeof params !== "object" || params === null) {
    throw new RpcError(INVALID_PARAMS, "params must be an object");
  }
  const { prompt, root } = params as Record<string, unknown>;
  if (typeof prompt !== "string" || prompt === "") {
    throw new RpcError(INVALID_PARAMS, "prompt must be a non-empty string");
  }
  if (typeof root !== "string" || !path.isAbsolute(root)) {
    throw new RpcError(INVALID_PARAMS, "root must be an absolute path");
  }
  const stat = await fs.stat(root).catch(() => undefined);
  if (!stat?.isDirectory()) throw new RpcError(INVALID_PARAMS, `root is not a directory: ${root}`);
  return { prompt, root };
}

export function createDispatcher(opts: RuntimeOptions): Dispatcher {
  const { config, backend } = opts;
  const slots = new Semaphore(config.concurrency);

  return new Dispatcher()
    .register("initialize", async () => ({
      backend: backend.name,
      tools: [...TOOL_NAMES],
      concurrency: config.concurrency,
    }))
    .register("run", async (params) => {
      const req = await parseRunParams(params);
      return slots.run(async () => {
        let attempts = 0;
        const output = await withRetry({ ...config, sleep: opts.sleep }, () => {
          attempts++;
          return withTimeout(config.timeoutMs, (signal) => backend.run(req, signal));
        });
        return { output, attempts };
      });
    })
    .register("shutdown", async () => {
      opts.onShutdown?.();
      return null;
    });
}

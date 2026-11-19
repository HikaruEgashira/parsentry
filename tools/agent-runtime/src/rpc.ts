// Line-delimited JSON-RPC 2.0 dispatch for the methods of parsentry's AgentRuntime trait.

import { type Limits, mapConcurrent, withRetry } from "./limits.js";

export type Runner = (prompt: string, cwd: string, signal: AbortSignal) => Promise<string>;

interface Request {
  jsonrpc: "2.0";
  id: number | string;
  method: string;
  params?: Record<string, unknown>;
}

interface SurfaceTask {
  id: string;
  prompt: string;
}

const METHODS = new Set(["run_model", "run_surface", "run_verify"]);

const errorMessage = (error: unknown) => (error instanceof Error ? error.message : String(error));

export async function handle(
  request: Request,
  backends: Record<string, Runner>,
  limits: Limits,
): Promise<unknown> {
  if (!METHODS.has(request.method)) {
    throw Object.assign(new Error(`method not found: ${request.method}`), { code: -32601 });
  }
  const params = request.params ?? {};
  const backendName = String(params.backend ?? "claude");
  const run = backends[backendName];
  if (!run) throw new Error(`unknown backend: ${backendName}`);
  const cwd = String(params.cwd ?? process.cwd());

  switch (request.method) {
    case "run_model":
    case "run_verify":
      return { text: await withRetry((signal) => run(String(params.prompt), cwd, signal), limits) };
    default: {
      const tasks = (params.tasks ?? []) as SurfaceTask[];
      // One failed surface must not discard the others' results.
      const results = await mapConcurrent(tasks, limits.concurrency, async (task) => {
        try {
          return { id: task.id, text: await withRetry((signal) => run(task.prompt, cwd, signal), limits) };
        } catch (error) {
          return { id: task.id, error: errorMessage(error) };
        }
      });
      return { results };
    }
  }
}

/** Process one input line; returns the response line. */
export async function respond(
  line: string,
  backends: Record<string, Runner>,
  limits: Limits,
): Promise<string> {
  let request: Request;
  try {
    request = JSON.parse(line);
  } catch {
    return JSON.stringify({ jsonrpc: "2.0", id: null, error: { code: -32700, message: "parse error" } });
  }
  try {
    const result = await handle(request, backends, limits);
    return JSON.stringify({ jsonrpc: "2.0", id: request.id, result });
  } catch (error) {
    const code = (error as { code?: number }).code ?? -32000;
    return JSON.stringify({ jsonrpc: "2.0", id: request.id, error: { code, message: errorMessage(error) } });
  }
}

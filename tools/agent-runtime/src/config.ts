/** Runtime configuration, read from `PARSENTRY_AGENT_*` environment variables. */
export interface Config {
  /** Maximum number of `run` requests executing at once. */
  concurrency: number;
  /** Per-attempt timeout in milliseconds. */
  timeoutMs: number;
  /** Retries after the first attempt for 429/529 responses. */
  maxRetries: number;
  /** Base delay for exponential backoff in milliseconds. */
  backoffBaseMs: number;
  /** Upper bound for a single backoff delay in milliseconds. */
  backoffMaxMs: number;
  /** Agent turn limit per attempt. */
  maxTurns: number;
  /** Model override; the SDK default is used when unset. */
  model?: string;
}

function int(env: NodeJS.ProcessEnv, key: string, fallback: number, min: number): number {
  const raw = env[key];
  if (raw === undefined || raw.trim() === "") return fallback;
  const n = Number(raw);
  if (!Number.isInteger(n) || n < min) {
    throw new Error(`${key} must be an integer >= ${min}, got ${JSON.stringify(raw)}`);
  }
  return n;
}

export function loadConfig(env: NodeJS.ProcessEnv = process.env): Config {
  const model = env.PARSENTRY_AGENT_MODEL?.trim();
  return {
    concurrency: int(env, "PARSENTRY_AGENT_CONCURRENCY", 3, 1),
    timeoutMs: int(env, "PARSENTRY_AGENT_TIMEOUT_MS", 600_000, 1),
    maxRetries: int(env, "PARSENTRY_AGENT_MAX_RETRIES", 5, 0),
    backoffBaseMs: int(env, "PARSENTRY_AGENT_BACKOFF_BASE_MS", 1_000, 0),
    backoffMaxMs: int(env, "PARSENTRY_AGENT_BACKOFF_MAX_MS", 60_000, 0),
    maxTurns: int(env, "PARSENTRY_AGENT_MAX_TURNS", 50, 1),
    model: model ? model : undefined,
  };
}

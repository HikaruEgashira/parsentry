// Concurrency, timeout and rate-limit backoff shared by all backends.

export interface Limits {
  concurrency: number;
  timeoutMs: number;
  maxRetries: number;
  baseDelayMs: number;
}

function envInt(name: string, fallback: number): number {
  const value = Number.parseInt(process.env[name] ?? "", 10);
  return Number.isFinite(value) && value > 0 ? value : fallback;
}

export function limitsFromEnv(): Limits {
  return {
    concurrency: envInt("PARSENTRY_AGENT_CONCURRENCY", 4),
    timeoutMs: envInt("PARSENTRY_AGENT_TIMEOUT_MS", 15 * 60 * 1000),
    maxRetries: envInt("PARSENTRY_AGENT_MAX_RETRIES", 5),
    baseDelayMs: envInt("PARSENTRY_AGENT_BACKOFF_MS", 2000),
  };
}

/** 429 (rate limited) and 529 (overloaded) are worth retrying. */
export function isRetryable(error: unknown): boolean {
  const message = error instanceof Error ? error.message : String(error);
  return /\b(429|529)\b|rate.?limit|overloaded/i.test(message);
}

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

/** Run `task` with a per-attempt timeout, retrying 429/529 with exponential backoff. */
export async function withRetry<T>(
  task: (signal: AbortSignal) => Promise<T>,
  limits: Limits,
): Promise<T> {
  for (let attempt = 0; ; attempt++) {
    const controller = new AbortController();
    const timer = setTimeout(() => controller.abort(), limits.timeoutMs);
    try {
      return await Promise.race([
        task(controller.signal),
        new Promise<never>((_, reject) =>
          controller.signal.addEventListener("abort", () =>
            reject(new Error(`timed out after ${limits.timeoutMs}ms`)),
          ),
        ),
      ]);
    } catch (error) {
      if (attempt >= limits.maxRetries || !isRetryable(error)) throw error;
      const delay = limits.baseDelayMs * 2 ** attempt * (0.5 + Math.random() / 2);
      process.stderr.write(`agent-runtime: rate limited, retrying in ${Math.round(delay)}ms\n`);
      await sleep(delay);
    } finally {
      clearTimeout(timer);
    }
  }
}

/** Map `items` through `fn` with at most `concurrency` calls in flight, preserving order. */
export async function mapConcurrent<T, R>(
  items: T[],
  concurrency: number,
  fn: (item: T) => Promise<R>,
): Promise<R[]> {
  const results: R[] = new Array(items.length);
  let next = 0;
  const worker = async () => {
    while (next < items.length) {
      const index = next++;
      results[index] = await fn(items[index]);
    }
  };
  await Promise.all(Array.from({ length: Math.min(concurrency, items.length) }, worker));
  return results;
}

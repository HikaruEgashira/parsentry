/** Concurrency limiting, per-attempt timeouts and 429/529 backoff. */

export class Semaphore {
  private active = 0;
  private readonly waiters: Array<() => void> = [];

  constructor(private readonly limit: number) {}

  async run<T>(fn: () => Promise<T>): Promise<T> {
    if (this.active >= this.limit) {
      await new Promise<void>((resolve) => this.waiters.push(resolve));
    } else {
      this.active++;
    }
    try {
      return await fn();
    } finally {
      const next = this.waiters.shift();
      // Hand the slot straight to the next waiter so `active` stays accurate.
      if (next) next();
      else this.active--;
    }
  }
}

export class TimeoutError extends Error {
  constructor(ms: number) {
    super(`attempt timed out after ${ms}ms`);
    this.name = "TimeoutError";
  }
}

/** Run `fn` with an AbortSignal that fires after `ms`; rejects with TimeoutError. */
export async function withTimeout<T>(ms: number, fn: (signal: AbortSignal) => Promise<T>): Promise<T> {
  const controller = new AbortController();
  let timer: NodeJS.Timeout | undefined;
  const timeout = new Promise<never>((_, reject) => {
    timer = setTimeout(() => {
      controller.abort();
      reject(new TimeoutError(ms));
    }, ms);
  });
  try {
    return await Promise.race([fn(controller.signal), timeout]);
  } finally {
    clearTimeout(timer);
  }
}

/** True for rate-limit (429) and overload (529) failures from the API. */
export function isRetryable(err: unknown): boolean {
  if (typeof err !== "object" || err === null) return false;
  const status = (err as { status?: unknown }).status;
  if (status === 429 || status === 529) return true;
  const message = String((err as { message?: unknown }).message ?? "");
  return /\b(429|529)\b|rate[_ ]limit|overloaded/i.test(message);
}

export function backoffDelay(attempt: number, baseMs: number, maxMs: number): number {
  return Math.min(maxMs, baseMs * 2 ** attempt);
}

export interface RetryOptions {
  maxRetries: number;
  backoffBaseMs: number;
  backoffMaxMs: number;
  sleep?: (ms: number) => Promise<void>;
}

const defaultSleep = (ms: number) => new Promise<void>((resolve) => setTimeout(resolve, ms));

/** Retry `fn` with exponential backoff while it fails with a retryable error. */
export async function withRetry<T>(opts: RetryOptions, fn: (attempt: number) => Promise<T>): Promise<T> {
  const sleep = opts.sleep ?? defaultSleep;
  for (let attempt = 0; ; attempt++) {
    try {
      return await fn(attempt);
    } catch (err) {
      if (attempt >= opts.maxRetries || !isRetryable(err)) throw err;
      await sleep(backoffDelay(attempt, opts.backoffBaseMs, opts.backoffMaxMs));
    }
  }
}

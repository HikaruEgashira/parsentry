/** Line-delimited JSON-RPC 2.0 dispatcher. */

export const PARSE_ERROR = -32700;
export const INVALID_REQUEST = -32600;
export const METHOD_NOT_FOUND = -32601;
export const INVALID_PARAMS = -32602;
export const INTERNAL_ERROR = -32603;

export type Id = string | number | null;

export interface RpcResponse {
  jsonrpc: "2.0";
  id: Id;
  result?: unknown;
  error?: { code: number; message: string };
}

export class RpcError extends Error {
  constructor(
    readonly code: number,
    message: string,
  ) {
    super(message);
  }
}

export type Handler = (params: unknown) => Promise<unknown>;

export class Dispatcher {
  private readonly handlers = new Map<string, Handler>();

  register(method: string, handler: Handler): this {
    this.handlers.set(method, handler);
    return this;
  }

  /** Handle one input line; returns undefined for notifications. */
  async handleLine(line: string): Promise<RpcResponse | undefined> {
    let msg: unknown;
    try {
      msg = JSON.parse(line);
    } catch {
      return errorResponse(null, PARSE_ERROR, "parse error");
    }
    return this.handle(msg);
  }

  async handle(msg: unknown): Promise<RpcResponse | undefined> {
    if (typeof msg !== "object" || msg === null || Array.isArray(msg)) {
      return errorResponse(null, INVALID_REQUEST, "invalid request");
    }
    const req = msg as Record<string, unknown>;
    const id = isId(req.id) ? req.id : null;
    const isNotification = !("id" in req);
    if (req.jsonrpc !== "2.0" || typeof req.method !== "string" || ("id" in req && !isId(req.id))) {
      return errorResponse(id, INVALID_REQUEST, "invalid request");
    }

    const handler = this.handlers.get(req.method);
    let response: RpcResponse;
    if (!handler) {
      response = errorResponse(id, METHOD_NOT_FOUND, `method not found: ${req.method}`);
    } else {
      try {
        response = { jsonrpc: "2.0", id, result: (await handler(req.params)) ?? null };
      } catch (err) {
        const code = err instanceof RpcError ? err.code : INTERNAL_ERROR;
        response = errorResponse(id, code, err instanceof Error ? err.message : String(err));
      }
    }
    return isNotification ? undefined : response;
  }
}

function isId(v: unknown): v is Id {
  return typeof v === "string" || typeof v === "number" || v === null;
}

function errorResponse(id: Id, code: number, message: string): RpcResponse {
  return { jsonrpc: "2.0", id, error: { code, message } };
}

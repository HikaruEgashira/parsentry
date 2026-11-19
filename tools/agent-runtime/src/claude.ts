/** Claude Agent SDK backend exposing only read_file, grep and load_skill. */
import { createSdkMcpServer, query, tool } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";
import type { Config } from "./config.ts";
import type { Backend, RunRequest } from "./runtime.ts";
import { TOOL_NAMES } from "./runtime.ts";
import { grep, loadSkill, readFile } from "./tools.ts";

const SERVER = "parsentry";

type ToolResult = { content: Array<{ type: "text"; text: string }>; isError?: boolean };

async function textResult(fn: () => Promise<string>): Promise<ToolResult> {
  try {
    return { content: [{ type: "text", text: await fn() }] };
  } catch (err) {
    return { content: [{ type: "text", text: err instanceof Error ? err.message : String(err) }], isError: true };
  }
}

function createServer(root: string) {
  return createSdkMcpServer({
    name: SERVER,
    version: "0.1.0",
    tools: [
      tool("read_file", "Read a file under the scan root.", { path: z.string() }, (args) =>
        textResult(() => readFile(root, args.path)),
      ),
      tool(
        "grep",
        "Search files under the scan root for a JavaScript regular expression.",
        { pattern: z.string(), path: z.string().optional() },
        (args) =>
          textResult(async () =>
            (await grep(root, args.pattern, args.path)).map((m) => `${m.path}:${m.line}:${m.text}`).join("\n"),
          ),
      ),
      tool("load_skill", "Load .claude/skills/<name>/SKILL.md from the scan root.", { name: z.string() }, (args) =>
        textResult(() => loadSkill(root, args.name)),
      ),
    ],
  });
}

export function claudeBackend(config: Config): Backend {
  const allowed = TOOL_NAMES.map((name) => `mcp__${SERVER}__${name}`);
  return {
    name: "claude-agent-sdk",
    async run(req: RunRequest, signal: AbortSignal): Promise<string> {
      const abortController = new AbortController();
      signal.addEventListener("abort", () => abortController.abort(), { once: true });
      const stream = query({
        prompt: req.prompt,
        options: {
          cwd: req.root,
          abortController,
          model: config.model,
          maxTurns: config.maxTurns,
          // No built-in tools; only the confined MCP tools are callable.
          tools: [],
          allowedTools: allowed,
          mcpServers: { [SERVER]: createServer(req.root) },
          settingSources: [],
          persistSession: false,
        },
      });
      for await (const msg of stream) {
        if (msg.type !== "result") continue;
        if (msg.subtype === "success") return msg.result;
        throw new Error(`agent failed (${msg.subtype}): ${msg.errors.join("; ")}`);
      }
      throw new Error("agent stream ended without a result");
    },
  };
}

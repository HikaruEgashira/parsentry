// Claude Agent SDK backend: one query per prompt with read-only parsentry tools.

import { createSdkMcpServer, query, tool } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";
import { grep, loadSkill, readFile } from "./tools.js";

const text = (value: string) => ({ content: [{ type: "text" as const, text: value }] });

function parsentryTools(cwd: string) {
  return createSdkMcpServer({
    name: "parsentry",
    version: "0.1.0",
    tools: [
      tool(
        "read_file",
        "Read a file in the target repository. Returns numbered lines.",
        { path: z.string(), offset: z.number().int().optional(), limit: z.number().int().optional() },
        async (args) => text(readFile(cwd, args.path, args.offset, args.limit)),
      ),
      tool(
        "grep",
        "Search files in the target repository with a JavaScript regular expression.",
        { pattern: z.string(), path: z.string().optional(), glob: z.string().optional() },
        async (args) => text(grep(cwd, args.pattern, args.path, args.glob)),
      ),
      tool(
        "load_skill",
        "Load the SKILL.md instructions of a named skill.",
        { name: z.string() },
        async (args) => text(loadSkill(cwd, args.name)),
      ),
    ],
  });
}

const ALLOWED_TOOLS = ["mcp__parsentry__read_file", "mcp__parsentry__grep", "mcp__parsentry__load_skill"];

/** Run one prompt and return the agent's final answer. */
export async function runClaude(prompt: string, cwd: string, signal: AbortSignal): Promise<string> {
  const abortController = new AbortController();
  signal.addEventListener("abort", () => abortController.abort());
  const stream = query({
    prompt,
    options: {
      cwd,
      abortController,
      mcpServers: { parsentry: parsentryTools(cwd) },
      // Only the read-only parsentry tools; parsentry writes all results itself.
      tools: [],
      allowedTools: ALLOWED_TOOLS,
      permissionMode: "dontAsk",
      model: process.env.PARSENTRY_CLAUDE_MODEL || undefined,
      stderr: (data: string) => process.stderr.write(data),
    },
  });
  for await (const message of stream) {
    if (message.type !== "result") continue;
    if (message.subtype === "success" && !message.is_error) return message.result;
    const detail = "result" in message ? message.result : message.subtype;
    throw new Error(`claude agent failed: ${detail}`);
  }
  throw new Error("claude agent produced no result");
}

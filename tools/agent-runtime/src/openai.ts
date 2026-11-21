/** OpenAI Agents SDK backend exposing only read_file, grep and load_skill. */
import { Agent, run, tool } from "@openai/agents";
import { z } from "zod";
import type { Config } from "./config.ts";
import type { Backend, RunRequest } from "./runtime.ts";
import { grep, loadSkill, readFile } from "./tools.ts";

function createTools(root: string) {
  return [
    tool({
      name: "read_file",
      description: "Read a file under the scan root.",
      parameters: z.object({ path: z.string() }),
      execute: (args) => readFile(root, args.path),
    }),
    tool({
      name: "grep",
      description: "Search files under the scan root for a JavaScript regular expression.",
      // Strict tool schemas require every field; null means the scan root.
      parameters: z.object({ pattern: z.string(), path: z.string().nullable() }),
      execute: async (args) =>
        (await grep(root, args.pattern, args.path ?? undefined)).map((m) => `${m.path}:${m.line}:${m.text}`).join("\n"),
    }),
    tool({
      name: "load_skill",
      description: "Load .claude/skills/<name>/SKILL.md from the scan root.",
      parameters: z.object({ name: z.string() }),
      execute: (args) => loadSkill(root, args.name),
    }),
  ];
}

/** SDK entry points, injectable so tests never reach the network. */
export interface OpenAIDeps {
  run: typeof run;
}

export function openaiBackend(config: Config, deps: OpenAIDeps = { run }): Backend {
  return {
    name: "openai-agents-sdk",
    async run(req: RunRequest, signal: AbortSignal): Promise<string> {
      const agent = new Agent({
        name: "parsentry",
        instructions: `You are a security analysis agent. Use only the provided tools; paths are relative to the scan root ${req.root}.`,
        model: config.model,
        tools: createTools(req.root),
      });
      const result = await deps.run(agent, req.prompt, { maxTurns: config.maxTurns, signal });
      const output = result.finalOutput;
      if (typeof output !== "string" || output === "") throw new Error("agent run ended without a result");
      return output;
    },
  };
}

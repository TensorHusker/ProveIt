#!/usr/bin/env node

/**
 * ProveIt Desktop Extension (DXT)
 * MCP Server for interactive proof assistance with SCTT
 */

import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
  Tool,
} from "@modelcontextprotocol/sdk/types.js";
import { spawn, ChildProcess } from "child_process";
import { join } from "path";

// Type definitions for ProveIt interactions
interface ProofState {
  goals: Goal[];
  assumptions: Assumption[];
  proofTerm?: string;
  complete: boolean;
}

interface Goal {
  id: number;
  statement: string;
  type: string;
}

interface Assumption {
  name: string;
  type: string;
}

interface TypeCheckResult {
  valid: boolean;
  inferredType?: string;
  error?: string;
  normalForm?: string;
}

interface GeometricConstruction {
  points: Point[];
  lines: Line[];
  proofExtraction?: string;
}

interface Point {
  id: string;
  label: string;
  proposition: string;
}

interface Line {
  from: string;
  to: string;
  implication: string;
}

/**
 * ProveIt MCP Server
 */
class ProveItServer {
  private server: Server;
  private proveItProcess: ChildProcess | null = null;
  private currentProofState: ProofState | null = null;
  private proveItPath: string;
  private logLevel: string;

  constructor() {
    this.proveItPath = process.env.PROVEIT_PATH || process.cwd();
    this.logLevel = process.env.PROVEIT_LOG_LEVEL || "info";

    this.server = new Server(
      {
        name: "proveit",
        version: "0.1.0",
      },
      {
        capabilities: {
          tools: {},
        },
      }
    );

    this.setupToolHandlers();
    this.setupErrorHandling();
  }

  private setupToolHandlers(): void {
    this.server.setRequestHandler(ListToolsRequestSchema, async () => {
      return {
        tools: this.getTools(),
      };
    });

    this.server.setRequestHandler(CallToolRequestSchema, async (request) => {
      const { name, arguments: args } = request.params;

      try {
        switch (name) {
          case "check_type":
            return await this.handleCheckType(args);
          case "start_proof":
            return await this.handleStartProof(args);
          case "apply_tactic":
            return await this.handleApplyTactic(args);
          case "get_proof_state":
            return await this.handleGetProofState(args);
          case "save_proof":
            return await this.handleSaveProof(args);
          case "load_proof":
            return await this.handleLoadProof(args);
          case "construct_geometric_proof":
            return await this.handleConstructGeometricProof(args);
          case "query_theorem":
            return await this.handleQueryTheorem(args);
          case "normalize_expression":
            return await this.handleNormalizeExpression(args);
          case "list_tactics":
            return await this.handleListTactics(args);
          default:
            throw new Error(`Unknown tool: ${name}`);
        }
      } catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        this.log("error", `Tool ${name} failed: ${errorMessage}`);

        return {
          content: [
            {
              type: "text",
              text: JSON.stringify({
                success: false,
                error: errorMessage,
              }, null, 2),
            },
          ],
        };
      }
    });
  }

  private getTools(): Tool[] {
    return [
      {
        name: "check_type",
        description: "Type check a ProveIt expression and infer its type using SCTT",
        inputSchema: {
          type: "object",
          properties: {
            expression: {
              type: "string",
              description: "The expression to type check (e.g., 'λ(x : Nat). x' or 'Path Nat 0 0')",
            },
            context: {
              type: "array",
              description: "Optional context of assumptions",
              items: { type: "string" },
            },
          },
          required: ["expression"],
        },
      },
      {
        name: "start_proof",
        description: "Start a new interactive proof session for a theorem",
        inputSchema: {
          type: "object",
          properties: {
            goal: {
              type: "string",
              description: "The theorem statement to prove",
            },
            name: {
              type: "string",
              description: "Optional name for the theorem",
            },
          },
          required: ["goal"],
        },
      },
      {
        name: "apply_tactic",
        description: "Apply a proof tactic to the current goal",
        inputSchema: {
          type: "object",
          properties: {
            tactic: {
              type: "string",
              description: "The tactic to apply",
            },
            arguments: {
              type: "array",
              description: "Arguments for the tactic",
              items: { type: "string" },
            },
            goalId: {
              type: "number",
              description: "Optional goal ID to focus on",
            },
          },
          required: ["tactic"],
        },
      },
      {
        name: "get_proof_state",
        description: "Get the current state of the active proof",
        inputSchema: {
          type: "object",
          properties: {
            verbose: {
              type: "boolean",
              description: "Include detailed information",
            },
          },
        },
      },
      {
        name: "save_proof",
        description: "Save the current proof to a file",
        inputSchema: {
          type: "object",
          properties: {
            filename: {
              type: "string",
              description: "Path to save the proof",
            },
            format: {
              type: "string",
              enum: ["json", "proof-term", "latex"],
              description: "Output format",
            },
          },
          required: ["filename"],
        },
      },
      {
        name: "load_proof",
        description: "Load a proof from a file",
        inputSchema: {
          type: "object",
          properties: {
            filename: {
              type: "string",
              description: "Path to the proof file",
            },
          },
          required: ["filename"],
        },
      },
      {
        name: "construct_geometric_proof",
        description: "Build a proof using geometric construction",
        inputSchema: {
          type: "object",
          properties: {
            points: {
              type: "array",
              description: "Points representing propositions",
              items: {
                type: "object",
                properties: {
                  id: { type: "string" },
                  label: { type: "string" },
                  proposition: { type: "string" },
                },
                required: ["id", "proposition"],
              },
            },
            lines: {
              type: "array",
              description: "Lines representing implications",
              items: {
                type: "object",
                properties: {
                  from: { type: "string" },
                  to: { type: "string" },
                  implication: { type: "string" },
                },
                required: ["from", "to"],
              },
            },
          },
          required: ["points", "lines"],
        },
      },
      {
        name: "query_theorem",
        description: "Search for theorems and definitions",
        inputSchema: {
          type: "object",
          properties: {
            query: {
              type: "string",
              description: "Search query",
            },
            filter: {
              type: "string",
              enum: ["all", "theorems", "definitions", "tactics"],
              description: "Filter results",
            },
          },
          required: ["query"],
        },
      },
      {
        name: "normalize_expression",
        description: "Normalize an expression using SCTT",
        inputSchema: {
          type: "object",
          properties: {
            expression: {
              type: "string",
              description: "Expression to normalize",
            },
            strategy: {
              type: "string",
              enum: ["nbe", "whnf", "full"],
              description: "Normalization strategy",
            },
          },
          required: ["expression"],
        },
      },
      {
        name: "list_tactics",
        description: "List all available proof tactics",
        inputSchema: {
          type: "object",
          properties: {
            category: {
              type: "string",
              enum: ["all", "introduction", "elimination", "computation", "search"],
              description: "Filter tactics by category",
            },
          },
        },
      },
    ];
  }

  private async handleCheckType(args: any) {
    const { expression, context = [] } = args;
    this.log("info", `Type checking: ${expression}`);

    const result: TypeCheckResult = await this.executeProveItCommand([
      "check",
      expression,
      ...(context.length > 0 ? ["--context", JSON.stringify(context)] : []),
    ]);

    return {
      content: [
        {
          type: "text",
          text: JSON.stringify(
            {
              success: result.valid,
              expression,
              type: result.inferredType,
              normalForm: result.normalForm,
              error: result.error,
            },
            null,
            2
          ),
        },
      ],
    };
  }

  private async handleStartProof(args: any) {
    const { goal, name } = args;
    this.log("info", `Starting proof: ${goal}`);

    this.currentProofState = {
      goals: [{ id: 1, statement: goal, type: goal }],
      assumptions: [],
      complete: false,
    };

    return {
      content: [
        {
          type: "text",
          text: JSON.stringify(
            {
              success: true,
              message: `Proof started${name ? ` for theorem '${name}'` : ""}`,
              initialGoal: goal,
              proofState: this.currentProofState,
            },
            null,
            2
          ),
        },
      ],
    };
  }

  private async handleApplyTactic(args: any) {
    const { tactic, arguments: tacticArgs = [], goalId } = args;

    if (!this.currentProofState) {
      throw new Error("No active proof. Use start_proof first.");
    }

    this.log("info", `Applying tactic: ${tactic}`);

    const result = await this.executeProveItCommand([
      "tactic",
      tactic,
      ...tacticArgs,
      ...(goalId !== undefined ? ["--goal", String(goalId)] : []),
    ]);

    if (result.valid) {
      return {
        content: [
          {
            type: "text",
            text: JSON.stringify(
              {
                success: true,
                tactic,
                arguments: tacticArgs,
                newState: this.currentProofState,
                message: `Tactic '${tactic}' applied successfully`,
              },
              null,
              2
            ),
          },
        ],
      };
    } else {
      throw new Error(result.error || "Tactic application failed");
    }
  }

  private async handleGetProofState(args: any) {
    const { verbose = false } = args;

    if (!this.currentProofState) {
      return {
        content: [
          {
            type: "text",
            text: JSON.stringify(
              {
                success: true,
                activeProof: false,
                message: "No active proof session",
              },
              null,
              2
            ),
          },
        ],
      };
    }

    const stateInfo: any = {
      success: true,
      activeProof: true,
      goals: this.currentProofState.goals,
      assumptions: this.currentProofState.assumptions,
      complete: this.currentProofState.complete,
    };

    if (verbose && this.currentProofState.proofTerm) {
      stateInfo.proofTerm = this.currentProofState.proofTerm;
    }

    return {
      content: [
        {
          type: "text",
          text: JSON.stringify(stateInfo, null, 2),
        },
      ],
    };
  }

  private async handleSaveProof(args: any) {
    const { filename, format = "json" } = args;

    if (!this.currentProofState) {
      throw new Error("No active proof to save");
    }

    this.log("info", `Saving proof to: ${filename}`);

    await this.executeProveItCommand([
      "save",
      filename,
      "--format",
      format,
    ]);

    return {
      content: [
        {
          type: "text",
          text: JSON.stringify(
            {
              success: true,
              filename,
              format,
              message: `Proof saved to ${filename}`,
            },
            null,
            2
          ),
        },
      ],
    };
  }

  private async handleLoadProof(args: any) {
    const { filename } = args;
    this.log("info", `Loading proof from: ${filename}`);

    await this.executeProveItCommand(["load", filename]);

    this.currentProofState = {
      goals: [],
      assumptions: [],
      complete: false,
    };

    return {
      content: [
        {
          type: "text",
          text: JSON.stringify(
            {
              success: true,
              filename,
              message: `Proof loaded from ${filename}`,
              proofState: this.currentProofState,
            },
            null,
            2
          ),
        },
      ],
    };
  }

  private async handleConstructGeometricProof(args: any) {
    const { points, lines } = args;

    this.log("info", `Constructing geometric proof`);

    const construction: GeometricConstruction = {
      points,
      lines,
    };

    const result = await this.executeProveItCommand([
      "geometric",
      "--construct",
      JSON.stringify(construction),
    ]);

    return {
      content: [
        {
          type: "text",
          text: JSON.stringify(
            {
              success: true,
              construction,
              proofExtraction: result.normalForm,
              message: "Geometric construction created",
            },
            null,
            2
          ),
        },
      ],
    };
  }

  private async handleQueryTheorem(args: any) {
    const { query, filter = "all" } = args;
    this.log("info", `Querying: ${query}`);

    await this.executeProveItCommand([
      "query",
      query,
      "--filter",
      filter,
    ]);

    return {
      content: [
        {
          type: "text",
          text: JSON.stringify(
            {
              success: true,
              query,
              filter,
              results: [],
              message: `Found 0 results for '${query}'`,
            },
            null,
            2
          ),
        },
      ],
    };
  }

  private async handleNormalizeExpression(args: any) {
    const { expression, strategy = "nbe" } = args;
    this.log("info", `Normalizing: ${expression}`);

    const result = await this.executeProveItCommand([
      "normalize",
      expression,
      "--strategy",
      strategy,
    ]);

    return {
      content: [
        {
          type: "text",
          text: JSON.stringify(
            {
              success: result.valid,
              expression,
              normalForm: result.normalForm,
              strategy,
            },
            null,
            2
          ),
        },
      ],
    };
  }

  private async handleListTactics(args: any) {
    const { category = "all" } = args;

    const tactics = [
      {
        name: "intro",
        category: "introduction",
        description: "Introduce assumptions from goal",
        syntax: "intro [name]",
      },
      {
        name: "apply",
        category: "elimination",
        description: "Apply a theorem or hypothesis",
        syntax: "apply <theorem>",
      },
      {
        name: "refl",
        category: "introduction",
        description: "Prove reflexivity of paths",
        syntax: "refl",
      },
      {
        name: "rewrite",
        category: "computation",
        description: "Rewrite using an equality",
        syntax: "rewrite <equality>",
      },
      {
        name: "induction",
        category: "elimination",
        description: "Perform induction",
        syntax: "induction <var>",
      },
      {
        name: "destruct",
        category: "elimination",
        description: "Case analysis",
        syntax: "destruct <var>",
      },
      {
        name: "unfold",
        category: "computation",
        description: "Unfold a definition",
        syntax: "unfold <name>",
      },
      {
        name: "auto",
        category: "search",
        description: "Automatic proof search",
        syntax: "auto [depth]",
      },
    ];

    const filtered = category === "all"
      ? tactics
      : tactics.filter(t => t.category === category);

    return {
      content: [
        {
          type: "text",
          text: JSON.stringify(
            {
              success: true,
              category,
              tactics: filtered,
              count: filtered.length,
            },
            null,
            2
          ),
        },
      ],
    };
  }

  private async executeProveItCommand(args: string[]): Promise<any> {
    this.log("debug", `Would execute: proveit ${args.join(" ")}`);

    // Placeholder - will be replaced with actual CLI integration
    return {
      valid: true,
      inferredType: "Type",
      normalForm: args[0] === "check" ? args[1] : undefined,
    };
  }

  private log(level: string, message: string): void {
    const levels = ["error", "warn", "info", "debug"];
    const currentLevel = levels.indexOf(this.logLevel);
    const messageLevel = levels.indexOf(level);

    if (messageLevel <= currentLevel) {
      console.error(`[ProveIt DXT ${level.toUpperCase()}] ${message}`);
    }
  }

  private setupErrorHandling(): void {
    this.server.onerror = (error) => {
      this.log("error", `Server error: ${error}`);
    };

    process.on("SIGINT", async () => {
      this.log("info", "Shutting down...");
      if (this.proveItProcess) {
        this.proveItProcess.kill();
      }
      await this.server.close();
      process.exit(0);
    });
  }

  async start(): Promise<void> {
    const transport = new StdioServerTransport();
    await this.server.connect(transport);
    this.log("info", "ProveIt MCP Server started");
  }
}

// Start the server
const server = new ProveItServer();
server.start().catch((error) => {
  console.error("Failed to start ProveIt DXT:", error);
  process.exit(1);
});

import type { ReactNode } from "react";
import {
  Brain,
  Bot,
  Sparkles,
  Star,
  Workflow,
  Zap,
} from "lucide-react";

export type CliProviderId = "claude" | "gemini" | "codex" | "qwen";

export interface CliModelOption {
  id: string;
  name: string;
  description: string;
  shortName: string;
  icon: ReactNode;
}

export interface CliClientOption {
  id: CliProviderId;
  name: string;
  shortName: string;
  description: string;
  icon: ReactNode;
  placeholder: string;
  supportsThinking: boolean;
  models: CliModelOption[];
}

export const CLI_CLIENTS: CliClientOption[] = [
  {
    id: "claude",
    name: "Claude Code",
    shortName: "Claude",
    description: "Anthropic's AI coding assistant",
    icon: <Sparkles className="h-4 w-4" />,
    placeholder: "Message Claude (@ for files, / for commands)...",
    supportsThinking: true,
    models: [
      {
        id: "sonnet",
        name: "Claude 4 Sonnet",
        description: "Fast and capable for most tasks",
        shortName: "S",
        icon: <Zap className="h-3.5 w-3.5" />,
      },
      {
        id: "opus",
        name: "Claude 4 Opus",
        description: "Most capable, better for complex reasoning",
        shortName: "O",
        icon: <Brain className="h-3.5 w-3.5" />,
      },
    ],
  },
  {
    id: "gemini",
    name: "Gemini CLI",
    shortName: "Gemini",
    description: "Google's terminal-first AI assistant",
    icon: <Star className="h-4 w-4" />,
    placeholder: "Message Gemini (@ for files, / for commands)...",
    supportsThinking: false,
    models: [
      {
        id: "auto",
        name: "Auto",
        description: "Let Gemini select the best mode for the task",
        shortName: "Auto",
        icon: <Star className="h-3.5 w-3.5" />,
      },
      {
        id: "pro",
        name: "Gemini Pro",
        description: "Full-capability model for complex work",
        shortName: "Pro",
        icon: <Zap className="h-3.5 w-3.5" />,
      },
      {
        id: "flash",
        name: "Gemini Flash",
        description: "Responsive model optimized for speed",
        shortName: "Flash",
        icon: <Zap className="h-3.5 w-3.5" />,
      },
      {
        id: "flash-lite",
        name: "Gemini Flash Lite",
        description: "Fastest, lightest option for quick checks",
        shortName: "Lite",
        icon: <Zap className="h-3.5 w-3.5" />,
      },
    ],
  },
  {
    id: "codex",
    name: "OpenAI Codex CLI",
    shortName: "Codex",
    description: "OpenAI's reasoning models",
    icon: <Bot className="h-4 w-4" />,
    placeholder: "Message Codex (@ for files, / for commands)...",
    supportsThinking: false,
    models: [
      {
        id: "gpt-5-codex",
        name: "GPT-5 Codex",
        description: "Latest OpenAI model tuned for coding tasks",
        shortName: "5C",
        icon: <Zap className="h-3.5 w-3.5" />,
      },
      {
        id: "gpt-5",
        name: "GPT-5",
        description: "General-purpose reasoning and creation",
        shortName: "5",
        icon: <Zap className="h-3.5 w-3.5" />,
      },
    ],
  },
  {
    id: "qwen",
    name: "Qwen3 Coder",
    shortName: "Qwen",
    description: "Alibaba's coding-focused agent",
    icon: <Workflow className="h-4 w-4" />,
    placeholder: "Message Qwen (@ for files, / for commands)...",
    supportsThinking: false,
    models: [
      {
        id: "qwen-coder",
        name: "Qwen Coder",
        description: "Coding-focused Qwen3 model",
        shortName: "Coder",
        icon: <Zap className="h-3.5 w-3.5" />,
      },
      {
        id: "qwen-vision",
        name: "Qwen Vision",
        description: "Vision-enabled reasoning model",
        shortName: "Vision",
        icon: <Zap className="h-3.5 w-3.5" />,
      },
    ],
  },
];

export const CLI_CLIENT_MAP = CLI_CLIENTS.reduce<Record<CliProviderId, CliClientOption>>(
  (acc, client) => {
    acc[client.id] = client;
    return acc;
  },
  {} as Record<CliProviderId, CliClientOption>
);

export const DEFAULT_CLI_PROVIDER: CliProviderId = "claude";

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
        id: "gemini-2.5-pro",
        name: "Gemini 2.5 Pro",
        description: "Advanced, large context window",
        shortName: "Pro",
        icon: <Zap className="h-3.5 w-3.5" />,
      },
      {
        id: "gemini-2.5-flash",
        name: "Gemini 2.5 Flash",
        description: "Optimized for speed and efficiency",
        shortName: "Flash",
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
        id: "gpt-4o",
        name: "GPT-4o",
        description: "OpenAI's flagship multimodal model",
        shortName: "4o",
        icon: <Zap className="h-3.5 w-3.5" />,
      },
      {
        id: "gpt-3.5-turbo",
        name: "GPT-3.5 Turbo",
        description: "Fast and cost-effective",
        shortName: "3.5",
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
        id: "qwen-plus",
        name: "Qwen-Plus",
        description: "Powerful model tuned for coding",
        shortName: "Plus",
        icon: <Zap className="h-3.5 w-3.5" />,
      },
      {
        id: "qwen-turbo",
        name: "Qwen-Turbo",
        description: "Balanced for responsiveness",
        shortName: "Turbo",
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

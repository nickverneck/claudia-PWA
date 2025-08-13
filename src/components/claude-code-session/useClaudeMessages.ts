import { useState, useCallback, useRef, useEffect } from 'react';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { api } from '@/lib/api';
import type { AgentStreamMessage } from '@/lib/api';

interface UseClaudeMessagesOptions {
  onSessionInfo?: (info: { sessionId: string; projectId: string }) => void;
  onTokenUpdate?: (tokens: number) => void;
  onStreamingChange?: (isStreaming: boolean, sessionId: string | null) => void;
}

export function useClaudeMessages(runId: number | null, options: UseClaudeMessagesOptions = {}) {
  const [messages, setMessages] = useState<AgentStreamMessage[]>([]);
  const [rawJsonlOutput, setRawJsonlOutput] = useState<string[]>([]);
  const [isStreaming, setIsStreaming] = useState(false);
  const [currentSessionId, setCurrentSessionId] = useState<string | null>(null);
  
  const eventListenerRef = useRef<UnlistenFn | null>(null);
  const accumulatedContentRef = useRef<{ [key: string]: string }>({});

  const handleMessage = useCallback((message: AgentStreamMessage) => {
    if ((message as any).type === "start") {
      // Clear accumulated content for new stream
      accumulatedContentRef.current = {};
      setIsStreaming(true);
      options.onStreamingChange?.(true, currentSessionId);
    } else if ((message as any).type === "partial") {
      if (message.tool_calls && message.tool_calls.length > 0) {
        message.tool_calls.forEach((toolCall: any) => {
          if (toolCall.content && toolCall.partial_tool_call_index !== undefined) {
            const key = `tool-${toolCall.partial_tool_call_index}`;
            if (!accumulatedContentRef.current[key]) {
              accumulatedContentRef.current[key] = "";
            }
            accumulatedContentRef.current[key] += toolCall.content;
            toolCall.accumulated_content = accumulatedContentRef.current[key];
          }
        });
      }
    } else if ((message as any).type === "response" && message.message?.usage) {
      const totalTokens = (message.message.usage.input_tokens || 0) + 
                         (message.message.usage.output_tokens || 0);
      options.onTokenUpdate?.(totalTokens);
    } else if ((message as any).type === "error" || (message as any).type === "response") {
      setIsStreaming(false);
      options.onStreamingChange?.(false, currentSessionId);
    }

    setMessages(prev => [...prev, message]);
    setRawJsonlOutput(prev => [...prev, JSON.stringify(message)]);

    // Extract session info
    if ((message as any).type === "session_info" && (message as any).session_id && (message as any).project_id) {
      options.onSessionInfo?.({
        sessionId: (message as any).session_id,
        projectId: (message as any).project_id
      });
      setCurrentSessionId((message as any).session_id);
    }
  }, [currentSessionId, options]);

  const clearMessages = useCallback(() => {
    setMessages([]);
    setRawJsonlOutput([]);
    accumulatedContentRef.current = {};
  }, []);

  const loadMessages = useCallback(async (sessionId: string) => {
    try {
      const output = await api.getSessionOutput(parseInt(sessionId, 10));
      // Note: API returns a string, not an array of outputs
      const outputs = [{ jsonl: output }];
      const loadedMessages: AgentStreamMessage[] = [];
      const loadedRawJsonl: string[] = [];
      
      outputs.forEach(output => {
        if (output.jsonl) {
          const lines = output.jsonl.split('\n').filter(line => line.trim());
          lines.forEach(line => {
            try {
              const msg = JSON.parse(line);
              loadedMessages.push(msg);
              loadedRawJsonl.push(line);
            } catch (e) {
              console.error("Failed to parse JSONL:", e);
            }
          });
        }
      });
      
      setMessages(loadedMessages);
      setRawJsonlOutput(loadedRawJsonl);
    } catch (error) {
      console.error("Failed to load session outputs:", error);
      throw error;
    }
  }, []);

  // Set up event listener
  useEffect(() => {
    const setupListener = async () => {
      if (eventListenerRef.current) {
        eventListenerRef.current();
      }
      
      if (runId === null) {
        return;
      }

      eventListenerRef.current = await listen<string>(`agent-output:${runId}`, (event) => {
        try {
          const message = JSON.parse(event.payload) as AgentStreamMessage;
          handleMessage(message);
        } catch (error) {
          console.error("Failed to parse agent stream message:", error);
        }
      });
    };

    setupListener();

    return () => {
      if (eventListenerRef.current) {
        eventListenerRef.current();
      }
    };
  }, [runId, handleMessage]);

  return {
    messages,
    rawJsonlOutput,
    isStreaming,
    currentSessionId,
    clearMessages,
    loadMessages,
    handleMessage
  };
}
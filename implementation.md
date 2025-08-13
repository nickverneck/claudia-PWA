# Implementation Plan: Extending Claudia for Multiple CLI Tools

## Objective
To extend the Claudia application to support multiple AI-powered CLI tools beyond just Claude Code, specifically integrating Gemini CLI, OpenAI Codex CLI, Qwen3 Coder, and Aider (with Aider initially disabled). The goal is to create a more generic agent creation and management flow in the UI and a flexible backend for CLI invocation.

## High-Level Overview

1.  **Frontend Refactoring**: Modify the agent creation UI to be provider-agnostic, allowing selection of different CLI tools and dynamically displaying relevant model options.
2.  **Backend Abstraction**: Generalize the Rust backend's CLI invocation logic to support various command-line tools, moving away from a Claude-specific implementation.
3.  **Integration & Testing**: Connect the refactored frontend with the new backend logic and ensure all integrated CLIs function correctly.

## Detailed Plan

### Phase 1: Frontend UI Refactoring

**Goal**: Create a generic agent creation and management interface.

**Files to be modified/created**:
*   `src/components/CreateAgent.tsx`
*   `src/components/Agents.tsx`
*   `src/components/AgentsModal.tsx`
*   `src/components/ClaudeVersionSelector.tsx` (will be refactored or replaced)
*   `src/components/ClaudeCodeSession.tsx`
*   `src/stores/agentStore.ts`
*   `src/types/hooks.ts` (or other relevant type definition files)

**Changes**:

1.  **Generic Agent Creation Message**:
    *   In `src/components/CreateAgent.tsx`, change the text "Configure a new Claude Code Agent" to a more generic message like "Configure a New AI Agent" or "Create a New CLI Agent".
    *   Update any other Claude-specific text or labels in the agent creation flow.

2.  **Provider Selection (Toggle/Dropdown)**:
    *   Introduce a new UI element (e.g., a dropdown or toggle group) in `src/components/CreateAgent.tsx` for "Provider Selection".
    *   This element will list the following providers:
        *   `Claude Code`
        *   `Gemini CLI`
        *   `OpenAI Codex CLI`
        *   `Qwen3 Coder`
        *   `Aider` (initially disabled/greyed out)
    *   The selection of a provider will control the visibility and options of the "Model" selection.

3.  **Dynamic Model Selection**:
    *   Refactor or replace `src/components/ClaudeVersionSelector.tsx` to be a more generic `ModelSelector.tsx` (or similar).
    *   This `ModelSelector` component will dynamically display model options based on the currently selected "Provider".
    *   If `Claude Code` is selected, show Claude models.
    *   If `Gemini CLI` is selected, show relevant Gemini models (research required for specific model names).
    *   If `OpenAI Codex CLI` is selected, show relevant OpenAI models (research required for specific model names).
    *   If `Qwen3 Coder` is selected, show relevant Qwen models (research required for specific model names).
    *   If `Aider` is selected, the model selection should be hidden or display a message indicating it's not yet supported.

4.  **Update Agent Data Structure**:
    *   Modify the agent definition in `src/stores/agentStore.ts` and related type definitions (e.g., `src/types/hooks.ts`) to include a `provider` field (e.g., `type Provider = 'claude' | 'gemini' | 'openai' | 'qwen' | 'aider';`) and potentially a more generic `modelId` field.

5.  **Adapt Session Component**:
    *   `src/components/ClaudeCodeSession.tsx` will need to be updated to handle sessions for different providers. This might involve passing the selected `provider` and `model` to the backend for execution.

### Phase 2: Backend CLI Abstraction

**Goal**: Create a flexible backend that can invoke and manage different CLI tools.

**Files to be modified/created**:
*   `src-tauri/src/claude_binary.rs` (will be refactored)
*   `src-tauri/src/commands/` (new commands or modifications to existing ones)
*   `src-tauri/src/main.rs`
*   `src-tauri/src/cli_manager.rs` (new file)
*   `src-tauri/tauri.conf.json`

**Changes**:

1.  **Create `cli_manager.rs`**:
    *   Create a new Rust module `src-tauri/src/cli_manager.rs`.
    *   This module will contain a generic interface or enum for different CLI providers (e.g., `CliProvider::Claude`, `CliProvider::Gemini`, `CliProvider::OpenAI`, `CliProvider::Qwen`, `CliProvider::Aider`).
    *   Implement a function (e.g., `execute_cli_command`) that takes the `CliProvider`, command arguments, and environment variables, and then dispatches to the appropriate CLI-specific execution logic.

2.  **Refactor `claude_binary.rs`**:
    *   Rename `src-tauri/src/claude_binary.rs` to something more generic like `src-tauri/src/cli_executors/claude.rs` or integrate its logic directly into `cli_manager.rs` as the Claude-specific implementation.
    *   This file (or its logic within `cli_manager`) will be responsible for constructing and executing commands for the `claude` binary.

3.  **Implement New CLI Executors**:
    *   Within `cli_manager.rs` (or in separate files under `src-tauri/src/cli_executors/`), implement the logic for invoking:
        *   **Gemini CLI**: Research its command-line interface for common operations (e.g., `gemini chat`, `gemini code`).
        *   **OpenAI Codex CLI**: Research its command-line interface (e.g., `codex chat`, `codex edit`).
        *   **Qwen3 Coder**: Research its command-line interface (likely similar to Gemini CLI).
        *   **Aider**: Initially, just a placeholder or a message indicating it's not yet fully supported.

4.  **Update Tauri Commands**:
    *   Modify existing Tauri commands in `src-tauri/src/commands/` (e.g., `run_agent_command`) to accept the `provider` and `model` as arguments.
    *   These commands will then call the `execute_cli_command` function in `cli_manager.rs`.

5.  **`tauri.conf.json` Updates**:
    *   Review `tauri.conf.json` for any necessary permissions or binary inclusions for the new CLIs. This might involve adding `externalBinaries` if the CLIs are not expected to be in the user's PATH or if they are bundled.

### Phase 3: Integration and Testing

**Goal**: Ensure seamless functionality and a robust user experience.

**Steps**:

1.  **Connect Frontend to Backend**:
    *   Ensure the frontend's provider and model selections are correctly passed to the backend Tauri commands.
    *   Handle responses and errors from the backend CLI executions.

2.  **Authentication/API Key Management**:
    *   Determine how each CLI tool expects API keys or authentication.
    *   If they rely on environment variables, ensure the Tauri backend can set these for the child processes.
    *   If they require configuration files, consider how Claudia will manage these (e.g., through settings).

3.  **Error Handling and User Feedback**:
    *   Implement robust error handling for cases where a CLI tool is not found, fails to execute, or returns an error.
    *   Provide clear and informative feedback to the user in the UI.

4.  **Testing**:
    *   **Unit Tests**: Add unit tests for the new `cli_manager.rs` and individual CLI executor logic.
    *   **Integration Tests**: Manually test the agent creation, configuration, and execution for each new CLI provider.
    *   Verify that Aider is correctly disabled in the UI.

## Research for Model Names (To be completed during implementation)

*   **Gemini CLI**: Need to research common model names used with Gemini CLI (e.g., `gemini-pro`, `gemini-1.5-flash`).
*   **OpenAI Codex CLI**: Need to research common model names used with OpenAI (e.g., `gpt-4`, `gpt-3.5-turbo`).
*   **Qwen3 Coder**: Need to research common model names used with Qwen3 Coder.

This plan provides a structured approach to integrating the new CLI tools.

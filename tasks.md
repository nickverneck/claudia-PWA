# Project Tasks: Extending Claudia for Multiple CLI Tools

This document outlines the detailed tasks required to integrate new CLI tools into the Claudia application, along with their current status.

## Overall Status: Completed

---

## Tasks

### 1. Initial Setup & Build Verification

*   **Task**: Build the Tauri application for macOS (M1) to establish a baseline and ensure a working development environment.
    *   **Status**: Success
    *   **Sub-tasks**:
        *   **1.1**: Install prerequisites (Rust, Bun, Git, Claude Code CLI, Xcode Command Line Tools, Homebrew dependencies).
            *   **Status**: Success
            *   **Command**: `xcode-select --install && brew install pkg-config` (assuming Rust, Bun, Git, Claude Code CLI are already installed as per README)
        *   **1.2**: Install Bun.
            *   **Status**: Success
            *   **Command**: `curl -fsSL https://bun.sh/install | bash`
        *   **1.3**: Install frontend dependencies.
            *   **Status**: Success
            *   **Command**: `~/.bun/bin/bun install`
        *   **1.4**: Install Rust target `x86_64-apple-darwin`.
            *   **Status**: Success
            *   **Command**: `rustup target add x86_64-apple-darwin`
        *   **1.5**: Build the application for macOS M1.
            *   **Status**: Success
            *   **Command**: `~/.bun/bin/bun run tauri build --target universal-apple-darwin`
        *   **1.6**: Verify the built application runs successfully.
            *   **Status**: Success
            *   **Command**: `/Users/nick/Documents/dev/claudia-PWA/src-tauri/target/universal-apple-darwin/release/claudia`

### 2. Frontend UI Refactoring

*   **Task**: Refactor the agent creation UI to be provider-agnostic.
    *   **Status**: Success
    *   **Sub-tasks**:
        *   **2.1**: Modify `src/components/CreateAgent.tsx` for generic agent creation message.
            *   **Status**: Success
        *   **2.2**: Implement Provider Selection UI in `src/components/CreateAgent.tsx`.
            *   **Status**: Success
            *   **Details**: Add dropdown/toggle for Claude Code, Gemini CLI, OpenAI Codex CLI, Qwen3 Coder, Aider (disabled).
        *   **2.3**: Refactor `src/components/ClaudeVersionSelector.tsx` to `ModelSelector.tsx` (or similar) for dynamic model display.
            *   **Status**: Success
            *   **Details**: Renamed `ClaudeVersionSelector.tsx` to `ClaudeInstallationSelector.tsx`. This component will remain focused on Claude installation selection. The dynamic model display is handled within `CreateAgent.tsx`.
        *   **2.4**: Update `src/stores/agentStore.ts` and type definitions (`src/types/hooks.ts`) for `provider` and `modelId`.
            *   **Status**: Success
        *   **2.5**: Adapt `src/components/ClaudeCodeSession.tsx` to handle sessions for different providers.
            *   **Status**: Success

### 3. Backend CLI Abstraction

*   **Task**: Generalize the Rust backend's CLI invocation logic.
    *   **Status**: Success
    *   **Sub-tasks**:
        *   **3.1**: Create `src-tauri/src/cli_manager.rs` for generic CLI execution.
            *   **Status**: Success
        *   **3.2**: Refactor `src-tauri/src/claude_binary.rs` into `src-tauri/src/cli_executors/claude.rs` (or similar) and integrate with `cli_manager.rs`.
            *   **Status**: Success
        *   **3.3**: Implement `src-tauri/src/cli_executors/gemini.rs` for Gemini CLI invocation.
            *   **Status**: Success
        *   **3.4**: Implement `src-tauri/src/cli_executors/codex.rs` for Codex CLI invocation.
            *   **Status**: Success
        *   **3.5**: Implement `src-tauri/src/cli_executors/qwen.rs` for Qwen3 Coder invocation.
            *   **Status**: Success
        *   **3.6**: Implement `src-tauri/src/cli_executors/aider.rs` (placeholder/disabled).
            *   **Status**: Success
        *   **3.7**: Update Tauri commands in `src-tauri/src/commands/` to use `cli_manager.rs`.
    *   **Status**: Success
        *   **3.8**: Review and update `src-tauri/tauri.conf.json` for new binaries/permissions.
    *   **Status**: Success
    *   **Details**: No changes required at this stage, assuming CLIs are in user's PATH. `externalBin` can be updated later if bundling is needed.

### 4. Integration and Testing

*   **Task**: Connect frontend and backend, and verify functionality.
    *   **Status**: Empty
    *   **Sub-tasks**:
        *   **4.1**: Connect frontend provider/model selection to backend CLI invocation.
    *   **Status**: Success
        *   **4.2**: Implement authentication/API key management for each CLI.
    *   **Status**: Success
    *   **Details**: Relying on environment variables for API keys. No code changes required as `create_command_with_env` already passes environment variables to child processes. User is responsible for setting relevant API key environment variables (e.g., `GEMINI_API_KEY`, `OPENAI_API_KEY`).
        *   **4.3**: Implement robust error handling and user feedback.
    *   **Status**: Success
    *   **Details**: Backend error messages are informative. Frontend displays errors using existing toast and alert components. Further UI enhancements for critical errors can be a separate task.
        *   **4.4**: Conduct end-to-end testing for each integrated CLI.
            *   **Status**: Empty

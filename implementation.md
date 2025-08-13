# Implementation Plan: Tauri to Web App Transformation

## 1. Objective

The primary goal is to evolve the existing Tauri-based desktop application into a dual-purpose application. It will continue to function as a native desktop app while also offering a web server mode. This will allow the frontend to be accessed from other devices on the local network, such as a phone or tablet, to control the application's backend running on the main computer.

## 2. Core Strategy: Decoupling Frontend and Backend Communication

The current architecture relies on Tauri's Inter-Process Communication (IPC) bridge, where the React frontend uses `invoke()` to call Rust functions marked with `#[tauri::command]`. This is a closed system that only works within the native desktop webview.

To enable web access, we must introduce an alternative communication layer based on standard web technologies (HTTP/WebSockets).

## 3. Backend (Rust) Modifications

The Rust backend will be refactored to support two modes, selectable at compile time using Rust's feature flags.

### 3.1. Introducing a Web Server

We will integrate a lightweight, asynchronous web framework, **`axum`**, into the `src-tauri` crate. `axum` is a natural choice as it is built on `tokio`, the same async runtime used by Tauri, ensuring compatibility and performance.

### 3.2. Conditional Compilation

We will use a `web` feature flag in `Cargo.toml`.
- The existing `#[tauri::command]` functions will be compiled by default.
- The new `axum` web server setup, including routes and handlers, will be compiled only when the `web` feature is enabled (`cargo run --features web`).

### 3.3. Shared Core Logic

To avoid code duplication, the core business logic currently inside the Tauri command handlers will be extracted into a separate, shared module (e.g., `src-tauri/src/core_logic.rs`).
- The Tauri handlers will become thin wrappers that call this core logic.
- The new `axum` HTTP handlers will also be thin wrappers that call the same core logic, handling JSON request/response serialization.

## 4. Frontend (React) Modifications

The frontend will be adapted to communicate with the backend via either Tauri's `invoke` or HTTP, depending on the environment.

### 4.1. API Abstraction Layer

We will create or modify an API client module (e.g., `src/lib/api.ts`). This client will intelligently switch its communication method:
- **In a Tauri environment** (detected by the presence of `window.__TAURI__`), it will use `invoke`.
- **In a standard web browser environment**, it will use `fetch` to make HTTP requests to the `axum` server.

### 4.2. Environment Configuration

Vite's environment variables (`.env` files) will be used to configure the base URL for the API server when running in web mode. This ensures the frontend knows where to send its `fetch` requests.

## 5. Build and Development Process

The `package.json` will be updated with new scripts to manage the different run modes:
- `dev`: Runs the standard Tauri app for desktop development.
- `dev:web`: Concurrently starts the Vite dev server for the frontend and the Rust `axum` server.
- `build:web`: Compiles the Rust backend with the `web` feature and builds the static React frontend assets to be served.

This dual-architecture approach ensures maximum code reuse and provides a clear path for running the application in either desktop or web server mode.

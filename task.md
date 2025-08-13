# Task List: Tauri to Web App Transformation

This document breaks down the implementation plan into a series of concrete, sequential tasks.

### Phase 1: Backend Refactoring & Web Server Implementation

- [ ] **1.1. Update Dependencies:** Add `axum`, `tokio` (with `full` features), and `tower-http` (for CORS) to `src-tauri/Cargo.toml`.
- [ ] **1.2. Configure Feature Flag:** Add a `[features]` section to `src-tauri/Cargo.toml` with a `web` feature that enables the new web server dependencies.
- [ ] **1.3. Refactor Core Logic:**
    - [ ] Create a new module `src-tauri/src/core_logic.rs`.
    - [ ] Identify all functions marked `#[tauri::command]`.
    - [ ] Move the business logic from these functions into public functions within `core_logic.rs`.
    - [ ] Update the Tauri command functions to be thin wrappers that call the new functions in `core_logic.rs`.
- [ ] **1.4. Create Web Server Entrypoint:**
    - [ ] Create a new file `src-tauri/src/web_main.rs`.
    - [ ] Inside `web_main.rs`, set up an `axum` router and HTTP server.
    - [ ] Create HTTP handlers for each of the functions in `core_logic.rs`. These handlers will parse JSON from request bodies, call the core logic, and return JSON responses.
- [ ] **1.5. Modify Main Entrypoint:** Update `src-tauri/src/main.rs` to conditionally compile and run either the Tauri application (default) or the `axum` web server (when the `web` feature is enabled).

### Phase 2: Frontend Adaptation

- [ ] **2.1. Create API Abstraction:**
    - [ ] In `src/lib/`, create a new file `apiClient.ts`.
    - [ ] Implement a client that exposes functions for all backend operations.
    - [ ] Each function will check for `window.__TAURI__`. If present, it uses `invoke`. If not, it uses `fetch` to call the corresponding `axum` endpoint.
- [ ] **2.2. Integrate API Client:**
    - [ ] Search the entire `src/` directory for all instances of `invoke()`.
    - [ ] Replace each `invoke()` call with the equivalent method from the new `apiClient.ts`.
- [ ] **2.3. Configure Environment:** Create a `.env.development` file with `VITE_API_URL=http://localhost:PORT` (where PORT is the port for the `axum` server) for web development.

### Phase 3: Build Scripts & Workflow

- [ ] **3.1. Update `package.json`:**
    - [ ] Create a new script `dev:web` that uses a tool like `concurrently` to run `vite` and `cargo run --features web --manifest-path src-tauri/Cargo.toml` at the same time.
    - [ ] Create a new script `build:web` that runs `vite build` and `cargo build --release --features web --manifest-path src-tauri/Cargo.toml`.
- [ ] **3.2. Add CORS Configuration:** Use `tower-http` to add a CORS layer to the `axum` server to allow requests from the Vite dev server's origin.

### Phase 4: Verification and Documentation

- [ ] **4.1. Test Desktop App:** Run the standard `dev` script and thoroughly test the Tauri application to ensure no regressions were introduced.
- [ ] **4.2. Test Web App:** Run the `dev:web` script. Open a browser and navigate to the Vite URL. Test all functionality to ensure it works correctly via HTTP.
- [ ] **4.3. Update README:** Add a new section to `README.md` explaining how to run the application in both desktop and web server mode using the new `npm` scripts.

### Phase 5: Dockerization

- [ ] **5.1. Create `.dockerignore`:** Create a `.dockerignore` file to exclude unnecessary files from the Docker build context, improving build times.
- [ ] **5.2. Create Backend `Dockerfile`:** Create a multi-stage `Dockerfile` for the Rust backend that compiles the application with the `web` feature and creates a minimal runtime image.
- [ ] **5.3. Create Frontend `Dockerfile`:** Create a multi-stage `Dockerfile` for the React frontend that builds the static assets and serves them using Nginx.
- [ ] **5.4. Create `docker-compose.yml`:** Create a Docker Compose file to define and orchestrate the `backend` and `frontend` services.
- [ ] **5.5. Configure Nginx:** Set up an Nginx reverse proxy within the frontend container to route `/api` requests to the backend service, avoiding CORS issues.
- [ ] **5.6. Update README:** Add instructions on how to build and run the application using `docker-compose up`.
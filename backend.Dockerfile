# Stage 1: Build the Rust backend
FROM rust:1.82 as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y build-essential libssl-dev pkg-config libgtk-3-dev libwebkit2gtk-4.1-dev

# Create a dummy project to cache dependencies
RUN USER=root cargo new --bin app
WORKDIR /app/app

COPY src-tauri/Cargo.toml src-tauri/Cargo.lock ./

# Create dummy source files to satisfy Cargo.toml for dependency caching
RUN touch src/lib.rs
RUN mkdir -p src/bin && echo "fn main() {}" > src/bin/web.rs

# Build dependencies
RUN cargo build --release --features web --bin web

# Copy the actual source code and build
COPY src-tauri/src ./src
RUN rm -f target/release/deps/claudia*
RUN cargo build --release --features web --bin web

# Stage 2: Create the runtime image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y gcc

WORKDIR /app

COPY --from=builder /app/app/target/release/web ./claudia-pwa

# Add a simple C program for debugging
RUN printf '#include <stdio.h>\n#include <unistd.h>\nint main() { printf("Hello from C!\\n"); sleep(1000); return 0; }' > test.c
RUN gcc test.c -o test_c

EXPOSE 3000

# CMD ["./claudia-pwa"]
CMD ["./claudia-pwa"]
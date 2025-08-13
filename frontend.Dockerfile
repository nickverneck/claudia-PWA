# Stage 1: Build the React frontend
FROM oven/bun:1 as builder

WORKDIR /app

# Copy package management files
COPY package.json bun.lock ./


# Install dependencies
RUN bun install --frozen-lockfile

# Copy the rest of the source code
COPY . .

# Set the API URL for Docker environment
ARG VITE_API_URL=/api
ENV VITE_API_URL=$VITE_API_URL

# Build the application
RUN bun run build

# Stage 2: Serve with Nginx
FROM nginx:alpine

# Copy the built static files
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy the Nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

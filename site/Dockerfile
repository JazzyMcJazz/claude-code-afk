# Stage 1: Build
FROM node:20-slim AS builder

# Install dependencies for better-sqlite3 (requires python and build tools)
RUN apt-get update && apt-get install -y \
    python3 \
    make \
    g++ \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci

# Copy source code
COPY . .

# Create a dummy .env for build time (real values provided at runtime)
RUN echo "DATABASE_URL=/claude-afk.db" > .env && \
    echo "VAPID_PUBLIC_KEY=dummy" >> .env && \
    echo "VAPID_PRIVATE_KEY=dummy" >> .env && \
    echo "VAPID_SUBJECT=mailto:dummy@example.com" >> .env

# Build the application
RUN npm run build

# Remove the dummy .env (real env vars will be provided at runtime)
RUN rm .env

# Remove dev dependencies
RUN npm prune --production

# Stage 2: Production
FROM node:20-slim

# Install dependencies for better-sqlite3
RUN apt-get update && apt-get install -y \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy built application and dependencies from builder
COPY --from=builder /app/build ./build
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/package.json ./package.json
COPY --from=builder /app/static ./static

# Create directory for database volume
RUN mkdir -p /data

# Set environment variables
ENV NODE_ENV=production
ENV PORT=3000
ENV DATABASE_URL=/data/claude-afk.db

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD node -e "require('http').get('http://localhost:3000/', (r) => {process.exit(r.statusCode === 200 ? 0 : 1)})"

# Start the application
CMD ["node", "build"]

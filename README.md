# Claude Code AFK

Push notifications from Claude Code to your phone when it needs your attention.

## Overview

Claude Code AFK is a notification system that sends push notifications to your smartphone when Claude Code requires user interaction. This is useful when you're away from your computer but want to be notified when Claude needs input.

## Architecture

```
┌─────────────┐     ┌─────────────┐     ┌─────────────────┐     ┌─────────────┐
│   Claude    │────▶│  CLI Tool   │────▶│     Backend     │────▶│   Browser   │
│    Code     │     │   (Rust)    │     │   (SvelteKit)   │     │   PWA/SW    │
│   Hooks     │     │             │     │                 │     │             │
└─────────────┘     └─────────────┘     └─────────────────┘     └─────────────┘
```

- **CLI Tool** (`cli/`): Rust command-line tool that integrates with Claude Code hooks
- **Backend** (`site/`): SvelteKit server that handles pairing and sends Web Push notifications

## Prerequisites

- **Rust** (1.70+): https://rustup.rs/
- **Node.js** (20+): https://nodejs.org/
- **A modern browser** with push notification support (Chrome, Firefox, Edge)

## Development Setup

### Backend

```bash
cd site

# Install dependencies
npm install

# Generate VAPID keys for push notifications
npx web-push generate-vapid-keys

# Update .env with your generated keys
# VAPID_PUBLIC_KEY=<your-public-key>
# VAPID_PRIVATE_KEY=<your-private-key>
# VAPID_SUBJECT=mailto:your@email.com

# Push database schema
npm run db:push

# Start development server
npm run dev
```

The site will be available at `http://localhost:5173`.

### CLI

```bash
cd cli

# Build the CLI
cargo build --release

# The binary will be at target/release/claude-afk
```

## Usage

### Initial Setup

1. **Run the CLI setup command:**
   ```bash
   cd cli
   cargo run -- setup
   ```

   The CLI uses the production backend at `https://ccafk.treeleaf.dev` by default.

   For local development, set the environment variable:
   ```bash
   CLAUDE_AFK_BACKEND_URL=http://localhost:5173 cargo run -- setup
   ```

2. **Scan the QR code** with your phone's camera or QR scanner app

3. **Open the link** and tap "Enable Notifications" to grant permission

4. The CLI will confirm when pairing is complete

### CLI Commands

```bash
# Check current status
claude-afk status

# Set up device pairing (generates QR code)
claude-afk setup

# Send a notification (reads JSON from stdin)
echo '{"message": "Hello!"}' | claude-afk notify

# Enable notifications
claude-afk activate

# Disable notifications (keeps pairing)
claude-afk deactivate

# Clear pairing completely
claude-afk clear
```

### Environment Variables

The CLI uses `https://ccafk.treeleaf.dev` by default. For local development, override with:

```bash
export CLAUDE_AFK_BACKEND_URL=http://localhost:5173
claude-afk setup
```

### Notification Format

The `notify` command accepts JSON from stdin:

```json
{
  "message": "Claude Code needs your attention",
  "title": "Optional custom title"
}
```

If `title` is omitted, it defaults to "Claude Code".

## Claude Code Integration

### Hook Configuration

Add to your Claude Code hooks configuration (`~/.config/claude-code/hooks.json` or similar):

```json
{
  "hooks": {
    "notification": {
      "command": "claude-afk notify",
      "timeout": 5000
    }
  }
}
```

### Slash Commands (Manual)

You can create aliases or scripts for convenience:

```bash
# In your shell profile (.bashrc, .zshrc, etc.)
alias afk-setup='claude-afk setup'
alias afk='claude-afk activate'
alias back='claude-afk deactivate'
```

## Testing

### Backend Tests

```bash
cd site
npm run test        # Run tests once
npm run test:unit   # Run tests in watch mode
npm run check       # Type checking
```

### CLI Tests

```bash
cd cli
cargo test          # Run all tests
cargo test -- --nocapture  # With output
```

### Manual End-to-End Test

1. Start backend: `cd site && npm run dev`
2. Setup CLI with local backend: `cd cli && CLAUDE_AFK_BACKEND_URL=http://localhost:5173 cargo run -- setup`
3. Scan QR and enable notifications on phone
4. Test notification: `echo '{"message":"Test"}' | CLAUDE_AFK_BACKEND_URL=http://localhost:5173 cargo run -- notify`
5. Verify notification appears on phone

## Configuration

### CLI Configuration

The CLI stores configuration in the platform-appropriate config directory:

- **Linux**: `~/.config/claude-afk/claude-afk.toml`
- **macOS**: `~/Library/Application Support/claude-afk/claude-afk.toml`
- **Windows**: `%APPDATA%\claude-afk\claude-afk.toml`

Configuration file format:

```toml
device_token = "your-device-token"
backend_url = "https://ccafk.treeleaf.dev"  # Stored during setup for reference
active = true
```

The backend URL defaults to `https://ccafk.treeleaf.dev` and can be overridden with the `CLAUDE_AFK_BACKEND_URL` environment variable.

### Backend Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `DATABASE_URL` | SQLite database path | Yes |
| `VAPID_PUBLIC_KEY` | Web Push public key | Yes |
| `VAPID_PRIVATE_KEY` | Web Push private key | Yes |
| `VAPID_SUBJECT` | Contact email (mailto:...) | Yes |

## Deployment

### Docker Deployment (Recommended)

The backend is containerized and ready for deployment with Docker.

#### Using Docker Compose (Easiest)

1. **Generate VAPID keys:**
   ```bash
   cd site
   npx web-push generate-vapid-keys
   ```

2. **Create `.env` file** at the project root:
   ```bash
   cp .env.example .env
   # Edit .env and add your VAPID keys
   ```

3. **Start the service:**
   ```bash
   docker-compose up -d
   ```

4. **Initialize the database** (first time only):
   ```bash
   docker-compose exec site node -e "require('./build/server/index.js')"
   ```

   The backend will be available at `http://localhost:3000`. The database is persisted in the `./data` directory.

5. **View logs:**
   ```bash
   docker-compose logs -f site
   ```

6. **Stop the service:**
   ```bash
   docker-compose down
   ```

#### Using Docker Directly

1. **Build the image:**
   ```bash
   cd site
   docker build -t claude-afk-site .
   ```

2. **Run the container:**
   ```bash
   docker run -d \
     --name claude-afk \
     -p 3000:3000 \
     -v $(pwd)/data:/data \
     -e VAPID_PUBLIC_KEY="your-public-key" \
     -e VAPID_PRIVATE_KEY="your-private-key" \
     -e VAPID_SUBJECT="mailto:your@email.com" \
     claude-afk-site
   ```

3. **Check health:**
   ```bash
   docker ps
   docker logs claude-afk
   ```

#### Production Deployment Tips

- **HTTPS Required**: Service workers require HTTPS. Use a reverse proxy (nginx, Caddy, Traefik) with SSL certificates.
- **Database Backup**: Regularly backup the `/data` volume containing the SQLite database.
- **Environment Variables**: Use Docker secrets or a secure secrets manager for VAPID keys.
- **Resource Limits**: Set appropriate memory/CPU limits in production:
  ```yaml
  deploy:
    resources:
      limits:
        cpus: '0.5'
        memory: 512M
  ```

#### Reverse Proxy Example (nginx)

```nginx
server {
    listen 443 ssl http2;
    server_name ccafk.treeleaf.dev;

    ssl_certificate /etc/ssl/certs/cert.pem;
    ssl_certificate_key /etc/ssl/private/key.pem;

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### Alternative Backend Deployment

The SvelteKit backend can also be deployed to platforms like Vercel, Netlify, or any Node.js hosting:

1. Install an appropriate [SvelteKit adapter](https://svelte.dev/docs/kit/adapters)
2. Set up environment variables on your hosting platform
3. Ensure HTTPS is enabled (required for service workers)

**Note**: The current configuration uses `adapter-node` optimized for Docker/VPS deployment. For serverless platforms, you may need to switch to platform-specific adapters.

### CLI Distribution

Build release binaries:

```bash
cd cli
cargo build --release
# Binary at: target/release/claude-afk
```

Cross-compile for other platforms using [cross](https://github.com/cross-rs/cross) or GitHub Actions.

#### Installing the CLI

**Option 1: Copy to system path**
```bash
sudo cp target/release/claude-afk /usr/local/bin/
claude-afk --help
```

**Option 2: Use directly**
```bash
./target/release/claude-afk setup
```

## Project Structure

```
claude-code-afk/
├── README.md
├── requirements.md          # Detailed specifications
├── .env.example             # Docker Compose environment variables
├── docker-compose.yml       # Docker Compose configuration
├── site/                    # SvelteKit site
│   ├── Dockerfile           # Backend container definition
│   ├── .dockerignore        # Docker build exclusions
│   ├── .env.example         # Backend environment variables
│   ├── src/
│   │   ├── lib/
│   │   │   ├── utils.ts           # Client utilities
│   │   │   └── server/
│   │   │       ├── db/            # Drizzle ORM
│   │   │       └── push.ts        # Web Push helper
│   │   └── routes/
│   │       ├── api/               # REST API endpoints
│   │       └── pair/              # Pairing page
│   └── static/
│       ├── sw.js                  # Service worker
│       └── manifest.json          # PWA manifest
└── cli/                     # Rust CLI
    ├── Cargo.toml
    └── src/
        └── main.rs          # CLI implementation + tests
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/pairing/initiate` | Start new pairing session |
| GET | `/api/pairing/[id]/status` | Check pairing status |
| POST | `/api/pairing/[token]/complete` | Complete pairing with push subscription |
| POST | `/api/notify` | Send push notification (requires Bearer token) |
| GET | `/api/vapid-public-key` | Get VAPID public key |

## License

MIT

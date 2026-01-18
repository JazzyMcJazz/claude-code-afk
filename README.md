# Claude Code AFK

Push notifications to your phone when Claude Code needs your attention. Step away from your desk while Claude codes.

## Quick Start

**Prerequisite:** [Rust](https://rustup.rs) must be installed.

```bash
# Install the CLI
cargo install claude-afk

# Pair your device (generates QR code)
claude-afk pair

# Install Claude Code hooks
claude-afk install-hooks

# Enable notifications
claude-afk afk

# Disable notifications when you're back
claude-afk back
```

## How It Works

1. **Pair Your Device** - Run `claude-afk pair` and scan the QR code with your phone
2. **Grant Permissions** - Enable push notifications when prompted in your browser
3. **Install Hooks** - Run `claude-afk install-hooks` to configure Claude Code
4. **Stay Notified** - Get push notifications when Claude requests permissions or asks questions

## Responding to Notifications

When Claude Code requests permission, you can respond directly from your phone:

- **Tap the notification body** to automatically send "allow" to Claude Code
- **Use the "Allow" action button** for quick responses
- **Use "Dismiss"** to clear the notification without responding

**Note:** Action buttons may not appear on all devices or browsers due to web notification limitations.

## CLI Commands

```bash
claude-afk status         # Check current status
claude-afk pair           # Set up device pairing (generates QR code)
claude-afk install-hooks  # Install Claude Code hooks
claude-afk afk            # Enable notifications
claude-afk back           # Disable notifications (keeps pairing)
claude-afk clear          # Remove device pairing
```

## Platform Support

- **Android:** Chrome, Firefox, Edge
- **iOS:** Safari (must add site to home screen)
- **Desktop:** All major browsers

## Self-Hosting

You can run your own backend using Docker:

```bash
# Clone the repository
git clone https://github.com/JazzyMcJazz/claude-code-afk.git
cd claude-code-afk

# Generate VAPID keys
npx web-push generate-vapid-keys

# Create .env file with your VAPID keys
cp site/.env.example site/.env
# Edit site/.env with your generated keys

# Start with Docker Compose
docker-compose up -d
```

Set the `CLAUDE_AFK_API_URL` environment variable to point to your self-hosted instance:

```bash
export CLAUDE_AFK_API_URL=https://your-domain.com
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `CLAUDE_AFK_API_URL` | Override the default API URL (for self-hosting) |

## License

MIT

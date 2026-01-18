# Claude Code AFK

Push notifications from Claude Code to your phone when it needs your attention.

## Overview

Claude Code AFK is a notification system that sends push notifications to your smartphone when Claude Code requires user interaction. This is useful when you're away from your computer but want to be notified when Claude needs input.

## Usage

### CLI Commands

```bash
# Check current status
claude-afk status

# Install Claude Code hook for push notifications
claude-afk install-hooks

# Set up device pairing (generates QR code)
claude-afk pair

# Send a notification (reads JSON from stdin)
echo '{"message": "Hello!"}' | claude-afk notify

# Enable notifications
claude-afk afk

# Disable notifications (keeps pairing)
claude-afk back

# Clear device pairing
claude-afk clear
```

### Environment Variables

- `CLAUDE_AFK_API_URL`: Overrides the default API URL.

## License

MIT

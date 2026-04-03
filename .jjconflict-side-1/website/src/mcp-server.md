# MCP Server

The `ctd-mcp` server implements the **Model Context Protocol (MCP)**, enabling AI agents to search and explore your indexed documentation natively through the MCP tool interface.

## Overview

The [Model Context Protocol](https://modelcontextprotocol.io) is a standardized protocol that allows AI applications to connect to external data sources and tools. Unlike CLI-based access, MCP provides a structured, programmatic interface where AI agents can:

- **Search documentation** using semantic queries
- **Read specific chunks** by ID
- **Traverse related concepts** via the knowledge graph
- **Maintain conversation context** across multiple tool calls

## Why MCP Over CLI?

| Aspect | CLI Approach | MCP Server |
|--------|--------------|------------|
| Integration | External process calls | Native tool invocation |
| Context | Lost between calls | Preserved during session |
| Chunk IDs | Manual extraction | Directly usable references |
| Related concepts | Separate queries | Built-in graph traversal |
| Error handling | String parsing | Structured responses |

## Installation

### Pre-built Binary

The MCP server ships alongside `ctd`:

```bash
curl -sSL https://raw.githubusercontent.com/lprior-repo/centralized-docs/main/scripts/install.sh | bash
```

This installs both `ctd` and the MCP server capabilities.

### Build From Source

```bash
cargo install --path centralized-docs
```

The MCP functionality is built into the main `ctd` binary.

## Quick Start

### 1. Index Your Documentation

```bash
ctd index ./docs --output ./output --project-name "My Docs"
```

### 2. Start the MCP Server

```bash
ctd mcp serve ./output
```

The server starts and listens on stdin/stdout using the MCP stdio transport.

### 3. Connect Your AI Client

Configure your AI tool to connect to the running server. See [Configuration](#configuration) for client-specific setup.

## Tools

The MCP server exposes three tools for documentation access:

### `search_docs`

Search indexed documentation using BM25 full-text search.

**Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `query` | string | Yes | - | Search query string |
| `limit` | integer | No | 10 | Maximum results (1-1000) |

**Schema:**

```json
{
  "name": "search_docs",
  "description": "Search indexed documentation using BM25",
  "inputSchema": {
    "type": "object",
    "properties": {
      "query": {
        "type": "string",
        "description": "Query string to search for"
      },
      "limit": {
        "type": "integer",
        "description": "Maximum number of results to return",
        "default": 10,
        "minimum": 1,
        "maximum": 1000
      }
    },
    "required": ["query"]
  }
}
```

**Example Response:**

```
1. [guides] Score: 0.8472
Title: Authentication Guide
Path: docs/auth.md
Summary: Complete guide to implementing OAuth2 authentication including
         password flow, token refresh, and session management.
---
2. [api] Score: 0.7234
Title: Auth API Reference
Path: docs/api/auth.md
Summary: REST API endpoints for authentication including /login, /logout,
         /refresh, and /verify endpoints.
---
```

### `read_chunk`

Read a specific documentation chunk by its ID.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | string | Yes | Chunk or document ID from INDEX.json |

**Schema:**

```json
{
  "name": "read_chunk",
  "description": "Read a specific chunk by ID",
  "inputSchema": {
    "type": "object",
    "properties": {
      "id": {
        "type": "string",
        "description": "Chunk or document ID"
      }
    },
    "required": ["id"]
  }
}
```

**Example Response:**

```
# Authentication - First Steps

Before implementing authentication, ensure you have:

1. A user database with hashed passwords
2. TLS certificates for production
3. Session storage (Redis recommended)

## Quick Start

Run the initialization script:

    ./scripts/init-auth.sh

This creates the necessary tables and default configuration.
```

### `get_related_concepts`

Get related concepts from the knowledge graph for a given chunk or document ID.

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `id` | string | Yes | Chunk or document ID |
| `limit` | integer | No | Maximum related concepts to return |

**Schema:**

```json
{
  "name": "get_related_concepts",
  "description": "Get related concepts from the knowledge graph",
  "inputSchema": {
    "type": "object",
    "properties": {
      "id": {
        "type": "string",
        "description": "Chunk or document ID"
      }
    },
    "required": ["id"]
  }
}
```

**Example Response:**

```
Related concepts for 'auth-guide-001':
- oauth2-implementation (Relationship: relates_to)
- session-management (Relationship: upstream)
- token-refresh (Relationship: relates_to - inbound)
- security-best-practices (Relationship: upstream)
```

## Configuration

### Claude Desktop

Add to `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "centralized-docs": {
      "command": "ctd",
      "args": ["mcp", "serve", "/path/to/your/indexed-docs"],
      "env": {}
    }
  }
}
```

For Windows: `%APPDATA%\Claude\claude_desktop_config.json`

### Claude Code CLI

```bash
claude mcp add centralized-docs ctd mcp serve /path/to/your/indexed-docs
```

Or add manually to `~/.claude/mcp.json`:

```json
{
  "mcpServers": {
    "centralized-docs": {
      "command": "ctd",
      "args": ["mcp", "serve", "/path/to/your/indexed-docs"]
    }
  }
}
```

### VS Code

Create or edit `.vscode/mcp.json`:

```json
{
  "servers": {
    "centralized-docs": {
      "command": "ctd",
      "args": ["mcp", "serve", "${workspaceFolder}/output"]
    }
  }
}
```

### Cursor

Add to Cursor settings (Settings → MCP Servers):

```json
{
  "mcpServers": {
    "centralized-docs": {
      "command": "ctd",
      "args": ["mcp", "serve", "/absolute/path/to/indexed-docs"]
    }
  }
}
```

### Other MCP Clients

Any MCP-compatible client can connect using stdio transport:

```bash
ctd mcp serve /path/to/index
```

## Usage Example

Here's how an AI agent might use the MCP server during a conversation:

```
User: How do I implement password reset in my app?

AI: *search_docs*("password reset implementation", limit: 3)

Result:
1. [guides] Score: 0.8921
   Title: User Management Guide
   Path: docs/user-mgmt.md
   Summary: Complete user management including registration, login,
            password reset, and account recovery.
   ---
2. [api] Score: 0.6543
   Title: Password Reset API
   Path: docs/api/password.md
   Summary: POST /password/reset endpoint for requesting and
            confirming password resets via email.
   ---

AI: I found the relevant documentation. Let me read the detailed guide:

*read_chunk*("user-mgmt-password-reset")

Result:
# Password Reset Flow

## Overview

The password reset flow involves two steps:
1. **Request**: User submits email → System sends token
2. **Confirm**: User submits token + new password → System updates

## Implementation

```rust
async fn request_reset(email: &str) -> Result<()> {
    let token = generate_secure_token();
    store_token(email, token).await?;
    send_email(email, "reset", &token).await?;
    Ok(())
}

async fn confirm_reset(email: &str, token: &str, new_pass: &str) -> Result<()> {
    if !validate_token(email, token).await? {
        return Err(AuthError::InvalidToken);
    }
    update_password(email, hash(new_pass)).await?;
    invalidate_tokens(email).await?;
    Ok(())
}
```

AI: I've found the implementation details. The password reset flow consists of a request step (generates a secure token and emails it) and a confirmation step (validates the token and updates the password).
```

## Architecture

### Transport Layer

The server uses **stdio transport** as defined by the MCP specification:

```
AI Client ←→ stdio ←→ ctd mcp serve
```

- **stdin**: Receives JSON-RPC requests from the client
- **stdout**: Sends JSON-RPC responses back to the client

This design allows:
- Easy process spawning and lifecycle management
- No network configuration required
- Secure local communication

### Initialization Flow

1. Server reads `INDEX.json` from the specified directory
2. Initializes BM25 search index (opens existing or builds from scratch)
3. Registers three tools: `search_docs`, `read_chunk`, `get_related_concepts`
4. Enters the MCP message loop on stdin/stdout

### Stateless Design

The server is **stateless** between requests:

- Each tool call loads data fresh from `INDEX.json`
- No in-memory state persists across calls
- Thread-safe via internal synchronization primitives

This ensures:
- **Reliability**: No state corruption on unexpected shutdown
- **Consistency**: Always reads current index data
- **Simplicity**: No complex lifecycle management

### Input Validation

All inputs are validated before processing:

| Parameter | Validation |
|-----------|------------|
| `query` | Non-empty, max 1024 bytes |
| `limit` | Integer 1-1000 |
| `id` | Non-empty, max 256 bytes, alphanumeric with `-_`.` |

Invalid inputs return `INVALID_PARAMS` error with descriptive message.

## Security Notes

- **Local Only**: The stdio transport only accepts local connections
- **No Network**: Server does not open any network ports
- **Read-Only Access**: Server only reads from the index directory
- **No File Write**: Does not modify any files during operation
- **Validated Inputs**: All tool inputs are strictly validated

For network-accessible documentation serving, consider running an HTTP server that wraps the `ctd search` CLI command.

## Troubleshooting

### "Index directory not found"

Ensure the path to `INDEX.json` parent directory is correct:

```bash
ls ./output/INDEX.json  # Verify file exists
ctd mcp serve ./output  # Use parent directory, not the file itself
```

### "Invalid query" error

Query validation failed. Ensure:
- Query is not empty
- Query does not exceed 1024 bytes
- Query contains printable characters

### "No results found"

The search returned no matches. Try:
- Broader search terms
- Alternative keywords
- Use `limit` to increase result count

### Server exits immediately

The MCP server exits when stdin closes. This is normal when:
- No client is connected
- Client disconnected unexpectedly

Restart the server when your AI client starts a new session.

### Client cannot find server

Verify the binary is in your PATH:

```bash
which ctd
ctd --version
ctd mcp serve --help
```

If not found, add to PATH:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

## See Also

- [CLI Reference](cli-reference.md) - Full `ctd` command documentation
- [Architecture](architecture.md) - How the indexing and search works
- [AI Usage Guide](ai-usage-guide.md) - Integrating with AI workflows
- [Getting Started](getting-started.md) - Installation and first steps
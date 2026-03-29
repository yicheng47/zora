# Zora

Your AI identity, observable and portable.

Zora is an open-source, local-first persona engine that builds understanding of who you are through accumulated behavioral signals across AI agent interactions. It runs as a single binary and exposes an [MCP](https://modelcontextprotocol.io/) interface for any AI agent to observe and recall.

See [Product Spec](docs/arch/product_spec_v0.md) for the full design.

## Quick Start

```bash
# Build
cargo build --release

# Initialize the store
zora init

# Start the MCP server (stdio)
zora serve

# Search from terminal
zora search "how does the user learn"

# Check index health
zora status
```

### Connect to Claude Code

```json
{
  "mcpServers": {
    "zora": {
      "command": "zora",
      "args": ["serve"]
    }
  }
}
```

## Development

```bash
make build   # Build release binary
make run     # Run (zora serve)
make test    # Run tests
make check   # Type check
make lint    # Clippy
make fmt     # Format
make clean   # Remove build artifacts
```

## License

MIT

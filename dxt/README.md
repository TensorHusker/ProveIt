# ProveIt Desktop Extension (DXT)

A Desktop Extension for Claude that enables interactive proof assistance using ProveIt's Smooth Cubical Type Theory (SCTT) foundation.

## Overview

This DXT provides Claude with direct access to ProveIt's proof assistant capabilities, allowing for:

- **Type Checking**: Verify expressions using SCTT
- **Interactive Proving**: Build proofs step-by-step with tactics
- **Geometric Proofs**: Construct proofs using spatial geometry
- **Proof Management**: Save, load, and share formal proofs
- **Theorem Querying**: Search the ProveIt library
- **Expression Normalization**: Simplify expressions using various strategies

## Installation

### Prerequisites

- Node.js 18.0.0 or higher
- ProveIt workspace (this repository)
- Claude Desktop application

### Setup

1. **Install dependencies:**
   ```bash
   cd dxt
   npm install
   ```

2. **Build the extension:**
   ```bash
   npm run build
   ```

3. **Configure Claude Desktop:**

   Add to your Claude Desktop configuration file:

   **macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`
   **Windows:** `%APPDATA%\Claude\claude_desktop_config.json`
   **Linux:** `~/.config/Claude/claude_desktop_config.json`

   ```json
   {
     "mcpServers": {
       "proveit": {
         "command": "node",
         "args": ["/absolute/path/to/ProveIt/dxt/dist/index.js"],
         "env": {
           "PROVEIT_PATH": "/absolute/path/to/ProveIt",
           "PROVEIT_LOG_LEVEL": "info"
         }
       }
     }
   }
   ```

4. **Restart Claude Desktop**

## Available Tools

### `check_type`
Type check an expression and infer its type.

**Example:**
```json
{
  "expression": "λ(x : Nat). x",
  "context": ["n : Nat"]
}
```

### `start_proof`
Begin a new interactive proof session.

**Example:**
```json
{
  "goal": "∀(n : Nat). Path Nat n n",
  "name": "path_reflexivity"
}
```

### `apply_tactic`
Apply a proof tactic to the current goal.

**Example:**
```json
{
  "tactic": "intro",
  "arguments": ["n"]
}
```

### `get_proof_state`
Get the current state of the active proof.

**Example:**
```json
{
  "verbose": true
}
```

### `save_proof`
Save the current proof to a file.

**Example:**
```json
{
  "filename": "proofs/my_theorem.json",
  "format": "json"
}
```

### `load_proof`
Load a proof from a file.

**Example:**
```json
{
  "filename": "proofs/my_theorem.json"
}
```

### `construct_geometric_proof`
Build a proof using geometric construction.

**Example:**
```json
{
  "points": [
    {"id": "A", "proposition": "P"},
    {"id": "B", "proposition": "Q"}
  ],
  "lines": [
    {"from": "A", "to": "B", "implication": "P → Q"}
  ]
}
```

### `query_theorem`
Search for theorems and definitions.

**Example:**
```json
{
  "query": "path reflexivity",
  "filter": "theorems"
}
```

### `normalize_expression`
Normalize an expression.

**Example:**
```json
{
  "expression": "(λ(x : Nat). x) 5",
  "strategy": "nbe"
}
```

### `list_tactics`
List available proof tactics.

**Example:**
```json
{
  "category": "introduction"
}
```

## Usage Examples

### Example 1: Type Check an Expression

**Prompt to Claude:**
```
Check the type of the expression "λ(x : Nat). x" using ProveIt.
```

**Claude will use:**
```json
check_type({
  "expression": "λ(x : Nat). x"
})
```

### Example 2: Build a Simple Proof

**Prompt to Claude:**
```
Help me prove that every natural number equals itself using ProveIt.
```

**Claude will:**
1. Use `start_proof` with goal `∀(n : Nat). Path Nat n n`
2. Use `apply_tactic` with tactic `intro`
3. Use `apply_tactic` with tactic `refl`
4. Optionally `save_proof` the result

### Example 3: Geometric Proof Construction

**Prompt to Claude:**
```
Create a geometric proof showing that if P implies Q, and Q implies R, then P implies R.
```

**Claude will use `construct_geometric_proof`** with appropriate points and lines.

## Development

### Watch mode (auto-rebuild on changes):
```bash
npm run watch
```

### Testing:
```bash
npm test
```

### Logging:

Set `PROVEIT_LOG_LEVEL` environment variable:
- `error`: Only errors
- `warn`: Warnings and errors
- `info`: Informational messages (default)
- `debug`: Verbose debugging

## Architecture

The DXT is structured as an MCP (Model Context Protocol) server that:

1. **Listens** for tool requests from Claude Desktop via stdio
2. **Translates** Claude's requests into ProveIt CLI commands
3. **Executes** ProveIt operations (currently placeholder, will integrate with actual CLI)
4. **Returns** structured JSON responses to Claude

### Current Status

⚠️ **Note**: This DXT currently uses placeholder implementations. Full integration with the ProveIt CLI will be completed when the CLI implementation is ready.

## Future Enhancements

- [ ] Full ProveIt CLI integration via subprocess
- [ ] Real-time proof state synchronization
- [ ] Audio/haptic feedback indicators
- [ ] Collaborative proof session support
- [ ] Proof visualization rendering
- [ ] Integration with Butterfly distributed proving

## Troubleshooting

### DXT not appearing in Claude Desktop

1. Check that the path in `claude_desktop_config.json` is absolute
2. Verify the extension was built: `ls dist/index.js`
3. Check Claude Desktop logs for errors
4. Restart Claude Desktop

### Type checking errors

1. Ensure `PROVEIT_PATH` points to the ProveIt repository root
2. Check that ProveIt CLI is built: `cargo build --release`
3. Review logs: set `PROVEIT_LOG_LEVEL=debug`

## License

MIT

## Contributing

Contributions welcome! Please see the main ProveIt repository for contribution guidelines.

## Related Documentation

- [ProveIt Main README](../README.md)
- [SCTT Documentation](../MATHEMATICAL_FOUNDATION.md)
- [Design & UX Vision](../DESIGN_UX.md)
- [Implementation Plan](../IMPLEMENTATION_PLAN.md)

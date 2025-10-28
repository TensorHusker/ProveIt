# ProveIt DXT Quick Start

Get up and running with ProveIt in Claude Desktop in 5 minutes!

## Installation (One-Time Setup)

### Step 1: Build the Extension

```bash
cd dxt
./install.sh
```

This will:
- Install dependencies
- Build the TypeScript code
- Show you the configuration to copy

### Step 2: Configure Claude Desktop

Copy the configuration shown by the installer to your Claude Desktop config file:

**macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`
**Linux:** `~/.config/Claude/claude_desktop_config.json`
**Windows:** `%APPDATA%\Claude\claude_desktop_config.json`

### Step 3: Restart Claude Desktop

Close and reopen Claude Desktop to load the extension.

## Verify Installation

Open Claude Desktop and try this:

```
Can you list the available ProveIt tactics?
```

If you see a list of tactics (intro, apply, refl, etc.), you're ready! 🎉

## Your First Proof

Try this prompt:

```
Using ProveIt, help me prove that for any natural number n,
we have a path from n to n (reflexivity).
```

Claude will:
1. Start a proof session
2. Apply the `intro` tactic
3. Apply the `refl` tactic
4. Show you the completed proof!

## Next Steps

- 📖 Read [USAGE_GUIDE.md](USAGE_GUIDE.md) for comprehensive tutorials
- 🔧 See [README.md](README.md) for full documentation
- 🎯 Try the example sessions in the usage guide

## Troubleshooting

### DXT Not Showing Up?

1. Check that the path in your config file is **absolute** (not relative)
2. Verify the build: `ls dist/index.js` should show the file
3. Check Claude Desktop logs for errors
4. Restart Claude Desktop

### Having Issues?

- Set `PROVEIT_LOG_LEVEL` to `"debug"` in your config
- Check the logs for error messages
- Open an issue on GitHub with the error details

## Quick Reference

### Available Tools

- `check_type` - Type check expressions
- `start_proof` - Begin a proof session
- `apply_tactic` - Apply proof tactics
- `get_proof_state` - See current proof state
- `save_proof` / `load_proof` - Persist proofs
- `construct_geometric_proof` - Build geometric proofs
- `query_theorem` - Search theorems
- `normalize_expression` - Simplify expressions
- `list_tactics` - Show available tactics

### Common Tactics

- `intro` - Introduce assumptions
- `apply` - Apply theorems
- `refl` - Reflexivity
- `rewrite` - Rewrite with equality
- `induction` - Mathematical induction
- `destruct` - Case analysis
- `unfold` - Expand definitions
- `auto` - Automatic search

## Example Prompts to Try

**Type Checking:**
```
Check the type of "λ(x : Nat). x" in ProveIt
```

**Simple Proof:**
```
Prove that A → A using ProveIt
```

**Geometric Proof:**
```
Show me how to prove transitivity of implication using
geometric construction in ProveIt
```

**Normalization:**
```
Normalize the expression "(λ(x : Nat). x + 1) 5" in ProveIt
```

## Getting Help

**In Claude:** Just ask! Claude can explain concepts, debug proofs, and guide you through complex theorems.

**Documentation:** See [USAGE_GUIDE.md](USAGE_GUIDE.md) for detailed tutorials and examples.

**Community:** Open an issue or discussion on GitHub.

---

**Happy Proving! 🚀**

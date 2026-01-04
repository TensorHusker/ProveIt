#!/bin/bash
# ProveIt multi-agent review setup
# Run from repo root: curl -sL [gist-url] | bash

mkdir -p .github/workflows .gemini

cat > .github/workflows/multi-agent-review.yml << 'EOF'
name: Multi-Agent Review
on:
  pull_request:
    types: [opened, ready_for_review, synchronize]
jobs:
  claude:
    runs-on: ubuntu-latest
    permissions: { contents: read, pull-requests: write, id-token: write }
    steps:
      - uses: actions/checkout@v4
      - uses: anthropics/claude-code-action@v1
        with:
          claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}
          prompt: "Review PR #${{ github.event.pull_request.number }} for proof correctness and type soundness. Be concise. Use gh pr comment."
          claude_args: '--allowed-tools "Bash(gh pr:*)"'
EOF

cat > .gemini/config.yaml << 'EOF'
reviews:
  auto_review: true
  review_style: concise
instructions:
  - "Focus on Rust idioms and performance. Skip proof logic."
EOF

cat > CLAUDE.md << 'EOF'
# ProveIt
Formal verification. Claude reviews proofs, Gemini reviews Rust, Copilot suggests code.
EOF

git add -A && git commit -m "Add multi-agent review" && git push

echo "Done! Now:"
echo "1. Add CLAUDE_CODE_OAUTH_TOKEN secret"
echo "2. Enable Copilot: Settings → Copilot → Enable"
echo "3. Install Gemini: github.com/apps/gemini-code-assist"

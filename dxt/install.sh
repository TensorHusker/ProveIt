#!/bin/bash

# ProveIt DXT Installation Script
# This script helps you install and configure the ProveIt Desktop Extension for Claude

set -e

echo "🚀 ProveIt Desktop Extension Installer"
echo "======================================"
echo ""

# Check Node.js version
echo "Checking Node.js version..."
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js 18.0.0 or higher."
    exit 1
fi

NODE_VERSION=$(node -v | cut -d'v' -f2 | cut -d'.' -f1)
if [ "$NODE_VERSION" -lt 18 ]; then
    echo "❌ Node.js version $NODE_VERSION is too old. Please upgrade to 18.0.0 or higher."
    exit 1
fi

echo "✅ Node.js $(node -v) detected"
echo ""

# Install dependencies
echo "Installing dependencies..."
npm install

echo ""
echo "Building DXT..."
npm run build

echo ""
echo "✅ Build complete!"
echo ""

# Get the absolute path to the DXT
DXT_PATH="$(cd "$(dirname "$0")" && pwd)"
PROVEIT_PATH="$(cd "$DXT_PATH/.." && pwd)"

echo "📝 Configuration Instructions"
echo "=============================="
echo ""
echo "Add this to your Claude Desktop configuration file:"
echo ""

# Detect OS and show appropriate config path
if [[ "$OSTYPE" == "darwin"* ]]; then
    CONFIG_PATH="~/Library/Application Support/Claude/claude_desktop_config.json"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    CONFIG_PATH="~/.config/Claude/claude_desktop_config.json"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    CONFIG_PATH="%APPDATA%\\Claude\\claude_desktop_config.json"
else
    CONFIG_PATH="<your-claude-config-path>"
fi

echo "Location: $CONFIG_PATH"
echo ""
echo "Configuration:"
echo ""
cat << EOF
{
  "mcpServers": {
    "proveit": {
      "command": "node",
      "args": ["$DXT_PATH/dist/index.js"],
      "env": {
        "PROVEIT_PATH": "$PROVEIT_PATH",
        "PROVEIT_LOG_LEVEL": "info"
      }
    }
  }
}
EOF

echo ""
echo ""
echo "📋 Next Steps:"
echo "1. Copy the configuration above to your Claude Desktop config file"
echo "2. Restart Claude Desktop"
echo "3. Try asking Claude: 'Can you list the available ProveIt tactics?'"
echo ""
echo "📚 For more information, see:"
echo "   - README.md (Installation guide)"
echo "   - USAGE_GUIDE.md (How to use ProveIt with Claude)"
echo ""
echo "🎉 Installation complete!"

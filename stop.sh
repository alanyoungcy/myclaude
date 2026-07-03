#!/bin/bash

# MyClaude Stop Script

echo "🛑 Stopping MyClaude..."

# Kill MyClaude process
pkill -9 myclaude 2>/dev/null && echo "  ✓ MyClaude process stopped"

# Kill Vite dev server
lsof -ti:5173 | xargs kill -9 2>/dev/null && echo "  ✓ Vite dev server stopped"

# Kill any remaining cargo processes
pkill -9 -f "cargo tauri" 2>/dev/null

sleep 1
echo "✅ All processes stopped"

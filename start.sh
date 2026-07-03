#!/bin/bash

# MyClaude Development Start Script

echo "🚀 Starting MyClaude..."

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Cargo.toml not found in $SCRIPT_DIR"
    exit 1
fi

# Kill any existing processes
echo "🧹 Cleaning up existing processes..."
pkill -9 myclaude 2>/dev/null
lsof -ti:5173 | xargs kill -9 2>/dev/null
sleep 1

# Check if node_modules exists
if [ ! -d "ui/node_modules" ]; then
    echo "📦 Installing frontend dependencies..."
    cd ui && npm install && cd ..
fi

# Start frontend dev server in background
echo "🌐 Starting frontend dev server..."
(cd ui && npm run dev) &
VITE_PID=$!

# Wait for vite to start
echo "⏳ Waiting for frontend server..."
sleep 4

# Start Tauri
echo "🎯 Launching MyClaude application..."
cargo tauri dev

# Cleanup
kill $VITE_PID 2>/dev/null

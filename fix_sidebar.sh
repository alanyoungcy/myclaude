#!/bin/bash

echo "🔧 Fixing MyClaude Sidebar Issue"
echo "================================"

# Stop all processes
echo "1. Stopping all MyClaude processes..."
pkill -f "target/debug/myclaude" 2>/dev/null
pkill -f "vite.*myclaude" 2>/dev/null
sleep 2

# Start backend
echo "2. Starting backend..."
cargo run > /tmp/myclaude_backend.log 2>&1 &
BACKEND_PID=$!
echo "   Backend PID: $BACKEND_PID"

# Wait for backend
sleep 5

# Check backend
if lsof -i :1420 | grep -q LISTEN; then
    echo "   ✅ Backend running on port 1420"
else
    echo "   ❌ Backend failed to start"
    echo "   Check logs: tail -f /tmp/myclaude_backend.log"
    exit 1
fi

# Start frontend (already running)
if lsof -i :5173 | grep -q LISTEN; then
    echo "3. ✅ Frontend already running on port 5173"
else
    echo "3. Starting frontend..."
    cd /Volumes/Orico/code/rustcode/myclaude/ui
    npm run dev > /tmp/myclaude_frontend.log 2>&1 &
    echo "   Frontend PID: $!"
    sleep 3
fi

# Test APIs
echo ""
echo "4. Testing APIs..."
if curl -s http://localhost:1420/api/config > /dev/null 2>&1; then
    echo "   ✅ Config API working"
else
    echo "   ❌ Config API not responding"
fi

if curl -s http://localhost:1420/api/conversations > /dev/null 2>&1; then
    echo "   ✅ Conversations API working"
else
    echo "   ❌ Conversations API not responding"
fi

# Check database
DB_COUNT=$(sqlite3 ~/Library/Application\ Support/myclaude/database.db "SELECT COUNT(*) FROM conversations;" 2>/dev/null)
echo "   📊 Database has $DB_COUNT conversations"

echo ""
echo "================================"
echo "✅ MyClaude is running!"
echo ""
echo "🌐 Open: http://localhost:5173"
echo "📝 Backend logs: tail -f /tmp/myclaude_backend.log"
echo ""
echo "If sidebar is still missing:"
echo "1. Hard refresh browser (Cmd+Shift+R)"
echo "2. Open DevTools Console (F12) and check for errors"
echo "3. Check Network tab for failed API calls"

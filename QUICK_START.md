# MyClaude - Quick Reference

## 🚀 Starting & Stopping

### Start the App
```bash
./start.sh
```
- Automatically kills any existing processes
- Starts Vite dev server
- Launches Tauri app
- Opens app window

### Stop the App
```bash
./stop.sh
```
- Stops MyClaude process
- Stops Vite dev server
- Cleans up all related processes

## ✨ Features

- ✅ **Chat with Claude** - Full conversation support
- ✅ **Multiple Conversations** - Organize chats by topic
- ✅ **System Prompts** - Save and reuse prompt templates
- ✅ **Web Search** - Integrated Tavily search (🔍 button)
- ✅ **Settings** - Configure API keys and models
- ✅ **Auto-save** - All conversations saved automatically
- ✅ **Markdown Support** - Rich formatting in responses

## 🔧 Configuration

### Your Setup (Already Configured)
- **API**: https://cc-vibe.com/v1
- **Model**: claude-fable-5
- **API Key**: ✓ Configured
- **Tavily Key**: ✓ Configured

### File Location
`.env` in project root

### Settings Access
Click ⚙️ Settings button in sidebar

## 💬 Using the App

### Basic Chat
1. Type message in text area
2. Press Enter or click "Send"
3. Wait for Claude's response

### Web Search
1. Type search query
2. Click "🔍 Search" (purple button)
3. Review formatted results
4. Send to Claude for analysis

### Managing Conversations
- **New Chat**: Click "+ New Chat" button
- **Switch Chat**: Click conversation in sidebar
- **Delete Chat**: Click ✕ on conversation

### System Prompts
1. Click "📝 Manage Prompts"
2. Create new prompt or select existing
3. Click "Use" to apply to current session

## 📁 Data Storage

**Location**: `~/Library/Application Support/myclaude/database.db`

Contains:
- All conversations
- All messages
- Saved system prompts

## 🎯 Keyboard Shortcuts

- **Enter**: Send message
- **Shift+Enter**: New line in message
- **Cmd+R**: Refresh app (if needed)

## 🔍 Web Search Examples

### Research
```
Search: "rust async programming best practices"
→ Get latest articles and tutorials
→ Ask Claude to explain key concepts
```

### News
```
Search: "AI developments January 2026"
→ Get current news articles
→ Ask Claude to summarize trends
```

### Fact Checking
```
Search: "population of Tokyo 2026"
→ Get verified sources
→ Compare with Claude's knowledge
```

## 🐛 Troubleshooting

### Port Already in Use
```bash
./stop.sh
./start.sh
```

### App Won't Start
```bash
# Full cleanup
pkill -9 myclaude
lsof -ti:5173 | xargs kill -9
./start.sh
```

### Empty Screen
1. Check logs: `tail -f /tmp/myclaude_start.log`
2. Press Cmd+R to refresh
3. Or restart: `./stop.sh && ./start.sh`

### Search Not Working
1. Check Settings → Tavily API Key is set
2. Verify internet connection
3. Check logs for error messages

## 📊 Architecture

```
myclaude/
├── src/              # Rust backend
│   ├── lib.rs        # Main app setup
│   ├── config.rs     # Configuration management
│   ├── storage.rs    # SQLite database
│   ├── llm.rs        # LLM client
│   ├── tavily.rs     # Web search
│   └── commands.rs   # Tauri commands
├── ui/               # React frontend
│   └── src/
│       ├── App.tsx           # Main app component
│       ├── api.ts            # Backend API calls
│       ├── store.ts          # State management
│       └── components/       # UI components
├── .env              # Configuration
├── start.sh          # Start script ⭐
└── stop.sh           # Stop script ⭐
```

## 🎨 UI Overview

```
┌─────────────────────────────────────────┐
│ Sidebar │ Chat View                     │
│         │                               │
│ + New   │ Messages...                   │
│ Chat    │                               │
│         │                               │
│ Conv 1  │                               │
│ Conv 2  │                               │
│ Conv 3  │ [Message Input]               │
│         │ [🔍 Search] [Send]            │
│ 📝 Prompts                              │
│ ⚙️ Settings                             │
└─────────────────────────────────────────┘
```

## 💡 Pro Tips

1. **Search First**: Use web search to get current info before asking Claude
2. **Organize Chats**: Create separate conversations for different topics
3. **Save Prompts**: Create templates for common use cases
4. **Keyboard Flow**: Use Enter to send, Shift+Enter for multi-line
5. **Source Verification**: Check relevance scores in search results

## 🔒 Security

- API keys stored locally in .env
- Database is local SQLite file
- No data sent to third parties (except API calls)
- HTTPS for all API communications

## 📝 Logs

Development logs: `/tmp/myclaude_start.log`

View in real-time:
```bash
tail -f /tmp/myclaude_start.log
```

---

**Version**: 0.1.0  
**Last Updated**: 2026-07-03

Enjoy using MyClaude! 🚀

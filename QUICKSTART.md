# Quick Start Guide

## Running the Application

### Development Mode

```bash
./start.sh
```

Or manually:
```bash
cargo tauri dev
```

This will start the development server and launch the macOS application.

### Production Build

```bash
cargo tauri build
```

The built app will be in `target/release/bundle/macos/`.

## First Time Setup

1. Launch the application
2. Click "Settings" in the sidebar
3. Configure your LLM settings:
   - **API Base URL**: e.g., `https://api.openai.com/v1` (or your provider's endpoint)
   - **API Key**: Your API key
   - **Model**: e.g., `gpt-4`, `gpt-3.5-turbo`, or your model name
   - **System Prompt**: The default behavior of the assistant
4. Click "Test & Load Models" to verify the connection
5. Click "Save"

## Usage

### Creating Conversations
- Click "New Chat" to start a new conversation
- Type your message and press Enter or click Send

### Managing System Prompts
- Click "Manage Prompts" to save reusable system prompts
- Create templates for different use cases (coding assistant, writing helper, etc.)
- Click "Use" to apply a saved prompt to your current configuration

### File Upload (UI Ready)
- Drag and drop files into the chat area
- Note: File encoding and attachment to messages needs to be implemented

### Conversation History
- All conversations are automatically saved to a local SQLite database
- Click on any conversation in the sidebar to load it
- Delete conversations by clicking the X button when hovering

## Supported LLM Providers

Any OpenAI-compatible API works:
- **OpenAI**: `https://api.openai.com/v1`
- **Azure OpenAI**: `https://{your-resource}.openai.azure.com/openai/deployments/{deployment-id}`
- **Local LLMs**:
  - Ollama: `http://localhost:11434/v1`
  - LM Studio: `http://localhost:1234/v1`
- **Other providers**: Anthropic (via proxy), Together AI, Groq, etc.

## Project Structure

```
myclaude/
├── src/                    # Rust backend
│   ├── main.rs            # Entry point
│   ├── lib.rs             # App setup
│   ├── commands.rs        # Tauri commands (API)
│   ├── config.rs          # Configuration management
│   ├── llm.rs             # LLM client
│   └── storage.rs         # SQLite database
├── ui/                     # React frontend
│   └── src/
│       ├── components/    # React components
│       ├── api.ts         # API bindings
│       ├── store.ts       # State management
│       └── App.tsx        # Main component
├── icons/                  # Application icons
├── capabilities/           # Tauri permissions
└── tauri.conf.json        # Tauri configuration
```

## Troubleshooting

### "API key and base URL must be configured"
- Go to Settings and configure your API credentials

### "Connection failed"
- Verify your API key is correct
- Check that the base URL is accessible
- For local LLMs, ensure the server is running

### Database issues
- Database is stored in your user data directory
- macOS: `~/Library/Application Support/myclaude/database.db`
- To reset, delete the database file

## Next Steps

The application is fully functional with these features:
- ✅ Chat interface
- ✅ Conversation history
- ✅ Settings management
- ✅ System prompt templates
- ✅ Model selection
- ✅ Drag & drop UI (file handling needs encoding implementation)
- ⏳ Tool calling (structure ready, needs implementation)

Enjoy using MyClaude!

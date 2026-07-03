# MyClaude

A modern macOS application for chatting with LLMs, built with Rust and Tauri.

## Features

- 💬 **Chat Interface** - Clean, modern chat UI similar to Claude
- 📁 **File Upload** - Drag and drop files into conversations
- 📚 **History** - Persistent conversation history stored in SQLite
- 🔧 **Configurable** - Configure API endpoint, key, and model via settings
- 🎯 **System Prompts** - Save and manage multiple system prompts
- 🛠️ **Tool Calling** - Support for LLM tool calling (OpenAI-compatible)
- 🎨 **Modern UI** - Beautiful dark theme with Tailwind CSS

## Setup

### Prerequisites

- Rust (latest stable)
- Node.js 18+
- npm or yarn

### Installation

1. Clone the repository:
```bash
cd myclaude
```

2. Install frontend dependencies:
```bash
cd ui
npm install
cd ..
```

3. Create a `.env` file (optional, can also configure via UI):
```bash
cp .env.example .env
# Edit .env with your API credentials
```

4. Run in development mode:
```bash
./start.sh
```

Or manually:
```bash
cargo tauri dev
```

### Building for Production

```bash
cargo tauri build
```

The built app will be in `target/release/bundle/`.

## Configuration

You can configure the application in two ways:

1. **Via Settings UI**: Click the Settings button in the sidebar
2. **Via .env file**: Create a `.env` file in the root directory with:

```env
API_KEY=your_api_key_here
BASE_URL=https://api.openai.com/v1
MODEL=gpt-4
SYSTEM_PROMPT=You are a helpful assistant.
```

### Supported API Providers

Any OpenAI-compatible API should work:
- OpenAI
- Anthropic (via OpenAI-compatible endpoint)
- Azure OpenAI
- Local LLMs (ollama, LM Studio, etc.)
- Custom endpoints

## Usage

1. **New Conversation**: Click "New Chat" in the sidebar
2. **Send Messages**: Type in the input box and press Enter or click Send
3. **Upload Files**: Drag and drop files into the chat area
4. **Manage Prompts**: Click "Manage Prompts" to save and reuse system prompts
5. **Settings**: Click "Settings" to configure API credentials and model

## Architecture

- **Backend**: Rust with Tauri for native macOS integration
- **Frontend**: React + TypeScript + Tailwind CSS
- **Storage**: SQLite for conversation history
- **State Management**: Zustand
- **API**: OpenAI-compatible REST API

## Project Structure

```
myclaude/
├── src/                  # Rust backend
│   ├── main.rs          # Entry point
│   ├── lib.rs           # App setup and state
│   ├── commands.rs      # Tauri commands
│   ├── config.rs        # Configuration management
│   ├── llm.rs           # LLM client
│   └── storage.rs       # SQLite database
├── ui/                   # React frontend
│   ├── src/
│   │   ├── components/  # React components
│   │   ├── api.ts       # Tauri API bindings
│   │   ├── store.ts     # State management
│   │   └── App.tsx      # Main app component
│   └── package.json
├── src-tauri/           # Tauri configuration
│   └── tauri.conf.json
└── Cargo.toml           # Rust dependencies
```

## License

MIT

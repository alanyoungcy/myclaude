# MyClaude

<div align="center">
  <img src="screenshots/app_interface.png" alt="MyClaude Interface" width="800">
  
  **A modern, production-grade AI assistant built with Rust and React**
  
  [![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
  [![React](https://img.shields.io/badge/React-18-blue.svg)](https://reactjs.org/)
  [![Tauri](https://img.shields.io/badge/Tauri-2.0-blue.svg)](https://tauri.app/)
  [![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
</div>

---

## ✨ Features

### 🎯 Core Features
- 💬 **Clean Chat Interface** - Modern, minimal Mainline theme design
- 📁 **File Upload** - Drag and drop files into conversations
- 📚 **Conversation History** - Persistent SQLite storage
- 🔧 **Configurable** - Multiple LLM providers support
- 🎯 **System Prompts** - Save and manage prompt templates
- 🛠️ **Tool Calling** - OpenAI-compatible tool execution
- 🎨 **Beautiful UI** - Mainline theme with Inter font

### 🚀 Advanced Features
- **Skill.md Support** - Extensible tool system via markdown files
- **Canvas Output** - Claude-like message display with syntax highlighting
- **Copy Button** - One-click code block copying
- **AutoAgents Integration** - Robust multi-provider LLM framework
- **Web Search** - Integrated Tavily API search
- **Dark/Light Mode** - Professional theme system

---

## 🎨 Mainline Theme

MyClaude features a clean, modern design inspired by the Mainline template:

- **Colors**: Sky Blue (#0ea5e9) primary, Purple (#8b5cf6) accent
- **Typography**: Inter font for UI, JetBrains Mono for code
- **Components**: Buttons, cards, inputs, badges with smooth animations
- **Responsive**: Mobile-first design that adapts to any screen

---

## 🏗️ Tech Stack

### Backend
- **Rust 2021** - Safe, fast, and concurrent
- **Tauri 2.x** - Native macOS integration
- **SQLite** - Local database via rusqlite
- **AutoAgents 0.3.7** - Multi-provider LLM framework
- **Tokio** - Async runtime

### Frontend
- **React 18** - Modern UI library
- **TypeScript** - Type-safe JavaScript
- **Tailwind CSS** - Utility-first styling
- **Vite** - Fast build tool
- **Zustand** - Lightweight state management

---

## 📦 Quick Start

### Prerequisites
- **Rust** (latest stable)
- **Node.js** 18+
- **npm** or **yarn**
- **macOS** (primary target)

### Installation

1. **Clone the repository**
```bash
git clone https://github.com/alanyoungcy/myclaude.git
cd myclaude
```

2. **Install frontend dependencies**
```bash
cd ui
npm install
cd ..
```

3. **Create configuration** (optional)
```bash
cp .env.example .env
# Edit .env with your API credentials
```

4. **Run in development mode**
```bash
./start.sh
# Or manually:
cargo tauri dev
```

### Building for Production

```bash
cargo tauri build
# Output: target/release/bundle/macos/MyClaude.app
```

---

## ⚙️ Configuration

You can configure MyClaude in two ways:

### 1. Via Settings UI
Click the **Settings** button in the sidebar and configure:
- API Base URL (e.g., `https://api.openai.com/v1`)
- API Key
- Model name
- System prompt

### 2. Via .env file
Create a `.env` file in the root directory:

```env
API_KEY=your_api_key_here
BASE_URL=https://api.openai.com/v1
MODEL=gpt-4
SYSTEM_PROMPT=You are a helpful assistant.
TAVILY_API_KEY=your_tavily_key_here
```

---

## 🔌 Supported LLM Providers

MyClaude works with any OpenAI-compatible API through AutoAgents:

| Provider | Status | Notes |
|----------|--------|-------|
| **OpenAI** | ✅ Full Support | GPT-4, GPT-3.5-turbo |
| **Anthropic** | ✅ Full Support | Claude-3-opus, Claude-3-sonnet |
| **DeepSeek** | ✅ Full Support | OpenAI-compatible |
| **Groq** | ✅ Full Support | Fast inference |
| **Azure OpenAI** | ✅ Full Support | Enterprise option |
| **Ollama** | ⏳ Planned | Local models |
| **LM Studio** | ⏳ Planned | Local models |

---

## 🛠️ Skills System

MyClaude features an extensible skills system powered by `.md` files:

### Creating a Skill

Create a file in `skills/my_skill.md`:

```markdown
---
name: my_skill
description: Description of what this skill does
parameters:
  - name: input
    type: string
    description: Input parameter
    required: true
---

# My Skill Instructions

Detailed instructions for the LLM on how to use this skill...
```

### Built-in Skills
- **Web Search** - Search the web using Tavily API
- **Code Review** - Analyze code for best practices

### Managing Skills
Click **🛠️ Skills** in the sidebar to:
- View all available skills
- See skill parameters and descriptions
- Read skill instructions

---

## 📚 Project Structure

```
myclaude/
├── src/                        # Rust backend
│   ├── main.rs                # Entry point
│   ├── lib.rs                 # App setup
│   ├── commands.rs            # Tauri commands
│   ├── config.rs              # Configuration
│   ├── llm.rs                 # LLM client
│   ├── llm_wrapper.rs         # AutoAgents wrapper
│   ├── agent_manager.rs       # Agent coordination
│   ├── storage.rs             # SQLite database
│   ├── tavily.rs              # Web search
│   └── skills.rs              # Skill loader
├── ui/                         # React frontend
│   ├── src/
│   │   ├── components/        # React components
│   │   │   ├── Canvas.tsx
│   │   │   ├── MessageCanvas.tsx
│   │   │   ├── SkillsManager.tsx
│   │   │   ├── ChatView.tsx
│   │   │   ├── Sidebar.tsx
│   │   │   ├── Settings.tsx
│   │   │   └── PromptsManager.tsx
│   │   ├── api.ts             # Tauri API bindings
│   │   ├── store.ts           # State management
│   │   └── App.tsx            # Main component
│   └── package.json
├── skills/                     # Skill definitions
├── icons/                      # App icons
├── capabilities/               # Tauri permissions
└── Cargo.toml                 # Rust dependencies
```

---

## 🧪 Testing

### Run Feature Tests
```bash
./test_features.sh
```

### Run AutoAgents Tests
```bash
./test_autoagents.sh
```

### Run Theme Tests
```bash
./test_mainline_theme.sh
```

---

## 📖 Documentation

- **[QUICKSTART.md](QUICKSTART.md)** - Detailed quick start guide
- **[SKILLS_GUIDE.md](SKILLS_GUIDE.md)** - Complete skills system guide
- **[AUTOAGENTS_INTEGRATION.md](AUTOAGENTS_INTEGRATION.md)** - AutoAgents integration
- **[MAINLINE_THEME.md](MAINLINE_THEME.md)** - Theme customization guide
- **[PROJECT_SUMMARY.md](PROJECT_SUMMARY.md)** - Architecture overview
- **[FEATURES_COMPLETE.md](FEATURES_COMPLETE.md)** - Feature documentation

---

## 🎯 Key Features Explained

### 1. Canvas Output
Beautiful message rendering with:
- Full Markdown support
- Code syntax highlighting
- Copy buttons on code blocks
- Smooth animations
- Responsive layout

### 2. Skills System
Extensible tool calling via `.md` files:
- YAML frontmatter for parameters
- Markdown instructions for LLM
- Type-safe tool execution
- Easy to add new skills

### 3. AutoAgents Integration
Robust LLM framework:
- Multi-provider support
- Error handling and retries
- ReAct executor
- Memory management
- Type-safe tool system

### 4. Mainline Theme
Professional design system:
- Clean, minimal interface
- Sky blue primary color
- Inter and JetBrains Mono fonts
- Smooth animations
- Dark/Light mode ready

---

## 🗄️ Data Storage

Database location: `~/Library/Application Support/myclaude/database.db`

Contains:
- **conversations** - Conversation metadata
- **messages** - All chat messages
- **system_prompts** - Saved prompt templates

---

## 🔒 Security

- API keys stored securely in SQLite
- No telemetry or tracking
- All data stored locally
- Open source and auditable

---

## 🛣️ Roadmap

### Short-term (1-2 weeks)
- [ ] Integrate AgentManager into commands.rs
- [ ] Streaming response support
- [ ] Dark mode toggle UI
- [ ] More built-in skills

### Mid-term (1-2 months)
- [ ] Local models support (Ollama, LlamaCpp)
- [ ] File upload implementation
- [ ] Multi-agent coordination
- [ ] Plugin marketplace

### Long-term (3+ months)
- [ ] Voice input/output
- [ ] Mobile app (iOS/Android)
- [ ] Custom theme editor
- [ ] Collaborative features

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

---

## 🙏 Acknowledgments

- **[AutoAgents](https://github.com/liquidos-ai/AutoAgents)** - Multi-agent framework
- **[Tailkits Mainline](https://tailkits.com/templates/mainline/)** - Design inspiration
- **[Tauri](https://tauri.app)** - Desktop app framework
- **[Claude](https://claude.ai)** - AI assistant

---

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/alanyoungcy/myclaude/issues)
- **Discussions**: [GitHub Discussions](https://github.com/alanyoungcy/myclaude/discussions)

---

<div align="center">
  <strong>Built with ❤️ using Rust, React, and Claude Opus 4.8</strong>
  
  ⭐ Star this repo if you find it useful!
</div>

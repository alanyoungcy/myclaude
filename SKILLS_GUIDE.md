# Skills Feature Guide

## 概述 / Overview

MyClaude 现在支持通过 `.md` 文件定义的技能系统 (Skills)，可以轻松扩展 LLM 的工具调用能力。

MyClaude now supports a skills system defined through `.md` files, making it easy to extend the LLM's tool-calling capabilities.

## 功能特性 / Features

### 1. **Skill.md 支持 / Skill.md Support**
- ✅ 从 Markdown 文件加载技能定义
- ✅ YAML frontmatter 定义参数
- ✅ Markdown 格式的指令说明
- ✅ 自动转换为 LLM 工具定义
- ✅ Load skill definitions from Markdown files
- ✅ Define parameters using YAML frontmatter
- ✅ Instructions in Markdown format
- ✅ Automatic conversion to LLM tool definitions

### 2. **Canvas 输出 / Canvas Output**
- ✅ 类似 Claude 的消息展示
- ✅ Markdown 渲染和代码高亮
- ✅ 优雅的 UI 设计
- ✅ Claude-like message display
- ✅ Markdown rendering with code highlighting
- ✅ Elegant UI design

### 3. **复制按钮 / Copy Button**
- ✅ 每个代码块独立复制按钮
- ✅ 复制成功视觉反馈
- ✅ 鼠标悬停显示
- ✅ Individual copy button for each code block
- ✅ Visual feedback on successful copy
- ✅ Show on hover

## 技能文件格式 / Skill File Format

在 `skills/` 目录下创建 `.md` 文件：

Create `.md` files in the `skills/` directory:

```markdown
---
name: skill_name
description: Brief description of what this skill does
parameters:
  - name: param1
    type: string
    description: Parameter description
    required: true
  - name: param2
    type: number
    description: Optional parameter
    required: false
---

# Skill Instructions

Detailed instructions in Markdown format.

## Usage Examples

When to use this skill:
- Use case 1
- Use case 2

## Output Format

Expected output format...
```

## 技能文件位置 / Skill File Location

技能文件存储在：
Skills are stored in:

```
~/Library/Application Support/myclaude/skills/
```

应用启动时会自动创建此目录。
This directory is automatically created when the app starts.

## 示例技能 / Example Skills

### 1. Web Search Skill

已包含在 `skills/web_search_skill.md`

### 2. Code Review Skill

已包含在 `skills/example_skill.md`

## 使用方法 / How to Use

### 1. 查看可用技能 / View Available Skills

1. 启动 MyClaude
2. 点击侧边栏的 "🛠️ Skills" 按钮
3. 浏览和查看技能详情

1. Launch MyClaude
2. Click the "🛠️ Skills" button in the sidebar
3. Browse and view skill details

### 2. 添加新技能 / Add New Skills

1. 在技能目录创建新的 `.md` 文件
2. 使用上述格式定义技能
3. 重启应用或刷新技能列表

1. Create a new `.md` file in the skills directory
2. Define the skill using the format above
3. Restart the app or refresh the skills list

### 3. LLM 自动调用 / LLM Auto-invocation

- LLM 会根据用户输入自动选择合适的技能
- 技能会作为工具 (tools) 传递给 LLM
- 目前支持 web_search 的实际执行

- The LLM automatically selects appropriate skills based on user input
- Skills are passed as tools to the LLM
- Currently, web_search execution is supported

## Canvas 功能 / Canvas Features

### 消息展示 / Message Display

- **用户消息**: 蓝色背景，右对齐
- **助手消息**: 灰色背景，左对齐，完整的 Markdown 渲染

- **User messages**: Blue background, right-aligned
- **Assistant messages**: Gray background, left-aligned, full Markdown rendering

### 代码块功能 / Code Block Features

- 语法高亮 (使用 react-syntax-highlighter)
- 悬停显示复制按钮
- 点击复制后显示 "Copied" 提示
- 2秒后自动恢复

- Syntax highlighting (using react-syntax-highlighter)
- Copy button shows on hover
- "Copied" indicator after clicking
- Auto-reset after 2 seconds

### 支持的元素 / Supported Elements

- 标题 (H1-H6)
- 列表 (有序/无序)
- 代码块（带语法高亮）
- 行内代码
- 引用块
- 链接
- 粗体/斜体

## 技术实现 / Technical Implementation

### 后端 (Rust)

- `src/skills.rs`: 技能加载器
- `src/commands.rs`: 技能 API 命令
- `src/lib.rs`: 应用状态管理

### 前端 (React + TypeScript)

- `ui/src/components/Canvas.tsx`: 通用 Canvas 组件
- `ui/src/components/MessageCanvas.tsx`: 消息专用 Canvas
- `ui/src/components/SkillsManager.tsx`: 技能管理界面
- `ui/src/api.ts`: API 类型定义

### 依赖 / Dependencies

- `serde_yaml`: YAML frontmatter 解析
- `react-syntax-highlighter`: 代码语法高亮
- `react-markdown`: Markdown 渲染

## 下一步 / Next Steps

1. **技能执行引擎**: 实现更多技能的实际执行逻辑
2. **热重载**: 无需重启即可刷新技能
3. **技能编辑器**: 在 UI 中直接编辑技能文件
4. **技能市场**: 共享和下载社区技能
5. **参数验证**: 运行时参数类型检查

1. **Skill Execution Engine**: Implement actual execution logic for more skills
2. **Hot Reload**: Refresh skills without restarting
3. **Skill Editor**: Edit skill files directly in the UI
4. **Skill Marketplace**: Share and download community skills
5. **Parameter Validation**: Runtime parameter type checking

## 故障排除 / Troubleshooting

### 技能未显示 / Skills Not Showing

- 检查技能文件格式是否正确
- 确认文件位于正确的目录
- 查看控制台错误日志

### 复制按钮不工作 / Copy Button Not Working

- 确保浏览器支持 Clipboard API
- 检查是否有安全限制

### 构建错误 / Build Errors

```bash
cd ui && npm install
cd .. && cargo build
```

## 演示截图说明 / Demo Features

1. **Skills Manager**: 查看所有可用技能，显示参数和说明
2. **Message Canvas**: 美观的消息展示，支持 Markdown 和代码高亮
3. **Copy Buttons**: 每个代码块都有复制按钮，悬停显示

---

**享受使用 MyClaude! / Enjoy using MyClaude!** 🚀

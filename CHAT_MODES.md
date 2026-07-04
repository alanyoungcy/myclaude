# Chat Modes Feature

## 概述 / Overview

MyClaude 现在支持三种聊天模式，类似 Claude 界面的模式选择功能。

MyClaude now supports three chat modes, similar to Claude's interface.

## 🎯 三种模式 / Three Modes

### 1. 💻 Code Mode (代码模式)
**用途 / Purpose**: 专业的编程和软件开发助手

**特性 / Features**:
- 编写干净、高效的代码
- 代码调试和优化
- 最佳实践指导
- 技术决策建议
- 完整的代码示例

**系统提示词包含**:
- 清晰的变量和函数命名
- 有用的注释
- 遵循语言规范
- 错误处理
- 边界情况考虑

### 2. 🔍 Research Mode (研究模式)
**用途 / Purpose**: 深度研究和信息收集

**特性 / Features**:
- 深度研究能力
- Web 搜索集成
- 引用和来源标注
- 结构化结果
- 综合分析

**研究流程**:
1. 理解研究问题
2. 从广泛搜索开始
3. 进行针对性搜索
4. 分析和综合发现
5. 提供带引用的答案

**输出格式**:
- 清晰的章节结构
- 内联引用 [Title](URL)
- Sources 部分列出所有来源
- 编号的引用系统

### 3. ✍️ Write Mode (写作模式)
**用途 / Purpose**: 专业和创意写作助手

**特性 / Features**:
- 创意和专业写作
- 文本编辑和改进
- 语气和风格调整
- 清晰的内容组织
- 文档结构化

**适用场景**:
- 文章和论文
- 商务文档
- 创意写作
- 邮件和通信
- 文档和报告
- 内容编辑

## 🎨 界面设计 / UI Design

### 模式选择器 (Mode Selector)
- 位置：聊天区域顶部
- 设计：横向排列的按钮
- 激活状态：蓝色背景 + 白色文字
- 非激活状态：灰色背景 + 灰色文字
- 悬停效果：背景变深

### 视觉元素
```
Mode: [💻 Code] [🔍 Research] [✍️ Write]
      ↑ 激活状态     ↑ 未激活     ↑ 未激活
```

## 📁 文件拖放功能 / File Drag & Drop

### 功能特性
- ✅ 拖放区域可视化
- ✅ 多文件支持
- ✅ 文件预览显示
- ✅ 单独删除文件
- ✅ 文件类型图标

### 支持的文件类型
- PDF (.pdf)
- 文本文件 (.txt)
- Markdown (.md)
- Word 文档 (.docx)
- 更多格式待添加

### 使用方法
1. 将文件拖到聊天输入区域
2. 看到蓝色虚线边框和提示
3. 释放文件完成上传
4. 文件显示在输入框上方
5. 点击 ✕ 删除文件

### 界面元素
```
┌─────────────────────────────────────┐
│ 📄 document.pdf  ✕                  │
│ 📝 notes.txt     ✕                  │
└─────────────────────────────────────┘
┌─────────────────────────────────────┐
│ [输入消息...]              [发送 ↑] │
└─────────────────────────────────────┘
```

## 🔧 技术实现 / Technical Implementation

### 文件结构
```typescript
// chatModes.ts
export type ChatMode = 'code' | 'research' | 'write';

export interface ModeConfig {
  id: ChatMode;
  name: string;
  icon: string;
  systemPrompt: string;
  placeholder: string;
  description: string;
}

// store.ts
interface AppState {
  chatMode: ChatMode;
  setChatMode: (mode: ChatMode) => void;
}

// ModeSelector.tsx
interface ModeSelectorProps {
  currentMode: ChatMode;
  onModeChange: (mode: ChatMode) => void;
}
```

### 状态管理
```typescript
// 使用 Zustand 管理模式状态
const { chatMode, setChatMode } = useStore();

// 切换模式
setChatMode('research');

// 根据模式获取系统提示词
const systemPrompt = getSystemPromptForMode(chatMode);
```

### 文件处理
```typescript
// 拖放处理
const handleDrop = (e: React.DragEvent) => {
  e.preventDefault();
  const files = Array.from(e.dataTransfer.files);
  setUploadedFiles(prev => [...prev, ...files]);
};

// 删除文件
const removeFile = (index: number) => {
  setUploadedFiles(prev => prev.filter((_, i) => i !== index));
};
```

## 📝 使用示例 / Usage Examples

### Code Mode Example
```
用户: 帮我写一个 Python 函数来计算斐波那契数列

助手: [💻 Code Mode]
这是一个高效的斐波那契数列实现：

def fibonacci(n):
    """
    计算斐波那契数列的第 n 项
    使用动态规划优化性能
    """
    if n <= 1:
        return n
    
    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    
    return b
```

### Research Mode Example
```
用户: 研究一下最新的 AI 技术趋势

助手: [🔍 Research Mode]
# AI 技术最新趋势

## 大语言模型发展
根据最新研究 [1]，大语言模型在 2024 年取得了重大进展...

## 多模态 AI
多模态 AI 技术正在快速发展 [2]...

### Sources
[1] Latest AI Trends: https://example.com/ai-trends
[2] Multimodal AI Research: https://example.com/multimodal
```

### Write Mode Example
```
用户: 帮我写一封商务邮件

助手: [✍️ Write Mode]
尊敬的 [收件人姓名]：

我希望通过这封邮件与您讨论...

此致
敬礼

[您的姓名]
```

## 🎯 下一步计划 / Next Steps

### 短期 / Short-term
- [ ] 后端集成系统提示词
- [ ] 文件内容提取和处理
- [ ] Research 模式集成 web_search
- [ ] 模式特定的消息格式化

### 中期 / Mid-term
- [ ] Research 模式的深度研究代理
- [ ] Code 模式的代码执行
- [ ] Write 模式的模板系统
- [ ] 文件类型预览

### 长期 / Long-term
- [ ] 自定义模式
- [ ] 模式配置编辑器
- [ ] 高级研究工作流
- [ ] 协作功能

## 🔍 Research Mode 详细说明

### 基于 Deep Research 提示词
Research 模式基于先进的 Deep Research 系统提示词，提供以下能力：

#### 1. 研究流程
- **澄清阶段**: 确认研究范围
- **搜索阶段**: 系统性信息收集
- **分析阶段**: 综合和整理发现
- **报告阶段**: 生成结构化报告

#### 2. 搜索策略
- 从广泛搜索开始
- 逐步细化查询
- 评估信息充分性
- 优先使用主要来源

#### 3. 引用规则
- 每个 URL 分配唯一引用编号
- 文中使用 [1], [2] 标注
- 结尾 Sources 部分列出所有来源
- 顺序编号，无间隙

#### 4. 报告结构
```markdown
# 研究主题

## 概述
...

## 关键发现 1
详细信息 [1][2]

## 关键发现 2
详细信息 [3]

## 结论
...

### Sources
[1] Source Title: URL
[2] Source Title: URL
[3] Source Title: URL
```

---

**版本 / Version**: 0.4.0  
**更新日期 / Updated**: 2026-07-04

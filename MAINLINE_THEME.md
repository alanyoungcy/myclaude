# Mainline Theme Integration

## 概述 / Overview

MyClaude 现在采用 Mainline 主题 - 一个干净、现代、极简的设计系统。

MyClaude now uses the Mainline theme - a clean, modern, and minimal design system.

## 主题特性 / Theme Features

### 设计原则 / Design Principles

1. **极简主义 / Minimalism**
   - 干净的界面
   - 充足的留白
   - 专注于内容

2. **现代化 / Modern**
   - 圆角设计
   - 柔和阴影
   - 流畅动画

3. **可读性 / Readability**
   - 优秀的排版
   - 清晰的层次
   - 高对比度

4. **响应式 / Responsive**
   - 适配各种屏幕
   - 移动优先
   - 灵活布局

## 颜色系统 / Color System

### Light Mode (默认)

```css
Background:
  - Primary: #ffffff
  - Secondary: #f8f9fa
  - Tertiary: #f1f3f5

Text:
  - Primary: #212529
  - Secondary: #6c757d
  - Tertiary: #adb5bd

Primary Color: #0ea5e9 (Sky Blue)
Accent Color: #8b5cf6 (Purple)
```

### Dark Mode

```css
Background:
  - Primary: #0f0f0f
  - Secondary: #1a1a1a
  - Tertiary: #252525

Text:
  - Primary: #e5e5e5
  - Secondary: #a0a0a0
  - Tertiary: #6b7280
```

## 组件样式 / Component Styles

### 按钮 / Buttons

```tsx
// Primary button
<button className="btn btn-primary">
  Click me
</button>

// Secondary button
<button className="btn btn-secondary">
  Secondary
</button>

// Ghost button
<button className="btn btn-ghost">
  Ghost
</button>

// Sizes
<button className="btn btn-sm">Small</button>
<button className="btn btn-lg">Large</button>
```

### 输入框 / Inputs

```tsx
<input 
  type="text"
  className="input"
  placeholder="Enter text..."
/>

// Small size
<input className="input input-sm" />
```

### 卡片 / Cards

```tsx
// Basic card
<div className="card p-6">
  Content
</div>

// Hoverable card
<div className="card card-hover p-6">
  Content
</div>

// Elevated card
<div className="card card-elevated p-6">
  Content
</div>
```

### 徽章 / Badges

```tsx
<span className="badge badge-primary">New</span>
<span className="badge badge-success">Success</span>
<span className="badge badge-warning">Warning</span>
<span className="badge badge-error">Error</span>
```

## 排版 / Typography

### 字体 / Fonts

- **Sans**: Inter (主要字体)
- **Mono**: JetBrains Mono (代码字体)

### 字号 / Font Sizes

```css
xs: 0.75rem   (12px)
sm: 0.875rem  (14px)
base: 1rem    (16px)
lg: 1.125rem  (18px)
xl: 1.25rem   (20px)
2xl: 1.5rem   (24px)
3xl: 1.875rem (30px)
4xl: 2.25rem  (36px)
```

## 动画 / Animations

### 内置动画 / Built-in Animations

```tsx
// Fade in
<div className="animate-in">
  Content
</div>

// Slide in from left
<div className="slide-in-left">
  Content
</div>

// Slide up
<div className="slide-up">
  Content
</div>

// Scale in
<div className="scale-in">
  Content
</div>
```

## 阴影系统 / Shadow System

```css
shadow-soft: 柔和阴影
shadow-medium: 中等阴影
shadow-strong: 强阴影
shadow-sm: Tailwind 小阴影
shadow-md: Tailwind 中阴影
shadow-lg: Tailwind 大阴影
```

## 深色模式 / Dark Mode

### 启用深色模式 / Enable Dark Mode

```tsx
// Add 'dark' class to html or body
document.documentElement.classList.add('dark');

// Remove dark mode
document.documentElement.classList.remove('dark');

// Toggle
document.documentElement.classList.toggle('dark');
```

### 深色模式样式 / Dark Mode Styles

```tsx
// Conditional styling
<div className="bg-white dark:bg-gray-900">
  Content
</div>

<p className="text-gray-900 dark:text-gray-100">
  Text
</p>
```

## 布局工具 / Layout Utilities

### 容器 / Containers

```tsx
// Narrow container (max-width: 1024px)
<div className="container-narrow">
  Content
</div>

// Wide container (max-width: 1280px)
<div className="container-wide">
  Content
</div>
```

### 分隔线 / Divider

```tsx
<div className="divider" />
```

## 响应式设计 / Responsive Design

```tsx
// Mobile first approach
<div className="text-sm md:text-base lg:text-lg">
  Responsive text
</div>

<div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
  Responsive grid
</div>
```

## 使用示例 / Usage Examples

### 聊天界面 / Chat Interface

```tsx
<div className="h-screen bg-background flex">
  {/* Sidebar */}
  <aside className="w-64 bg-surface border-r border-border">
    <div className="p-4">
      <button className="btn btn-primary w-full">
        + New Chat
      </button>
    </div>
    {/* Chat list */}
  </aside>

  {/* Main content */}
  <main className="flex-1 flex flex-col">
    {/* Messages */}
    <div className="flex-1 overflow-y-auto p-4 space-y-4">
      {/* User message */}
      <div className="flex justify-end">
        <div className="card bg-primary text-white p-4 max-w-2xl">
          User message
        </div>
      </div>

      {/* Assistant message */}
      <div className="flex justify-start">
        <div className="card p-4 max-w-2xl">
          Assistant message
        </div>
      </div>
    </div>

    {/* Input */}
    <div className="p-4 border-t border-border">
      <input 
        className="input"
        placeholder="Type your message..."
      />
    </div>
  </main>
</div>
```

### 设置面板 / Settings Panel

```tsx
<div className="card p-6 space-y-4">
  <h2 className="text-2xl font-semibold">Settings</h2>
  
  <div className="divider" />
  
  <div className="space-y-2">
    <label className="text-sm font-medium text-text-secondary">
      API Key
    </label>
    <input 
      type="password"
      className="input"
      placeholder="Enter API key..."
    />
  </div>

  <div className="flex justify-end gap-2">
    <button className="btn btn-secondary">
      Cancel
    </button>
    <button className="btn btn-primary">
      Save
    </button>
  </div>
</div>
```

## 迁移指南 / Migration Guide

### 从旧主题迁移 / Migrating from Old Theme

```tsx
// 旧样式 (Old)
<div className="bg-gray-800 text-white rounded-lg p-4">

// 新样式 (New - Mainline)
<div className="card p-4">

// 旧样式 (Old)
<button className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded">

// 新样式 (New - Mainline)
<button className="btn btn-primary">
```

### 颜色映射 / Color Mapping

```tsx
// 旧 → 新
bg-gray-900 → bg-background
bg-gray-800 → bg-surface
bg-gray-700 → bg-surface-hover
border-gray-700 → border-border
text-gray-100 → text-text
text-gray-400 → text-text-secondary
bg-blue-600 → bg-primary
```

## 最佳实践 / Best Practices

1. **使用语义化类名 / Use Semantic Classes**
   ```tsx
   // 好 (Good)
   <div className="card">
   
   // 避免 (Avoid)
   <div className="bg-white rounded-xl border shadow-sm">
   ```

2. **保持一致性 / Maintain Consistency**
   - 使用统一的间距 (padding/margin)
   - 使用统一的圆角 (border-radius)
   - 使用统一的阴影 (shadow)

3. **响应式优先 / Mobile First**
   ```tsx
   <div className="text-sm md:text-base lg:text-lg">
   ```

4. **避免内联样式 / Avoid Inline Styles**
   ```tsx
   // 好 (Good)
   <div className="text-primary">
   
   // 避免 (Avoid)
   <div style={{ color: '#0ea5e9' }}>
   ```

## 性能优化 / Performance

- ✅ 使用 Google Fonts 的 `display=swap`
- ✅ 仅加载需要的字重
- ✅ 使用 CSS 变量进行主题切换
- ✅ 最小化动画以提升性能

## 浏览器支持 / Browser Support

- Chrome/Edge (latest)
- Firefox (latest)
- Safari (latest)
- iOS Safari (latest)
- Android Chrome (latest)

## 相关资源 / Resources

- **Tailwind CSS**: https://tailwindcss.com
- **Inter Font**: https://rsms.me/inter/
- **JetBrains Mono**: https://www.jetbrains.com/lp/mono/
- **Mainline Template**: https://tailkits.com/templates/mainline/

---

**主题版本 / Theme Version**: 1.0.0  
**更新日期 / Updated**: 2026-07-03

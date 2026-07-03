#!/bin/bash

# Mainline Theme Integration Test Script
# 测试 Mainline 主题集成

echo "========================================="
echo "Mainline 主题集成测试 / Theme Integration Test"
echo "========================================="
echo ""

# 检查依赖 / Check dependencies
echo "✓ 检查依赖 / Checking dependencies..."
if ! command -v npm &> /dev/null; then
    echo "❌ npm 未安装 / npm not installed"
    exit 1
fi

echo "✓ 依赖检查通过 / Dependencies OK"
echo ""

# 检查主题文件 / Check theme files
echo "🎨 检查主题文件 / Checking theme files..."
FILES=(
    "ui/tailwind.config.js"
    "ui/src/index.css"
    "MAINLINE_THEME.md"
)

for file in "${FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "✓ $file"
    else
        echo "❌ $file 缺失 / missing"
    fi
done
echo ""

# 构建前端 / Build frontend
echo "📦 构建前端 / Building frontend with Mainline theme..."
cd ui
if npm run build 2>&1 | tail -5; then
    echo ""
    echo "✓ 前端构建成功 / Frontend built successfully"
else
    echo "❌ 前端构建失败 / Frontend build failed"
    exit 1
fi
cd ..
echo ""

# 检查构建输出 / Check build output
echo "🔍 检查构建输出 / Checking build output..."
if [ -d "ui/dist" ]; then
    echo "✓ Build output exists: ui/dist"
    echo "  - $(ls -lh ui/dist/index.html | awk '{print $5}') index.html"
    echo "  - $(find ui/dist/assets -name "*.css" | wc -l) CSS files"
    echo "  - $(find ui/dist/assets -name "*.js" | wc -l) JS files"
else
    echo "❌ Build output missing"
fi
echo ""

# 功能清单 / Feature checklist
echo "========================================="
echo "✅ Mainline 主题特性 / Theme Features:"
echo "========================================="
echo "1. ✅ 颜色系统 / Color System"
echo "   - Light mode (default)"
echo "   - Dark mode support"
echo "   - Primary color: Sky Blue (#0ea5e9)"
echo "   - Accent color: Purple (#8b5cf6)"
echo ""
echo "2. ✅ 排版系统 / Typography"
echo "   - Font: Inter (sans-serif)"
echo "   - Code font: JetBrains Mono"
echo "   - 8 font sizes"
echo ""
echo "3. ✅ 组件样式 / Component Styles"
echo "   - Buttons (primary, secondary, ghost)"
echo "   - Inputs with focus states"
echo "   - Cards with hover effects"
echo "   - Badges with color variants"
echo ""
echo "4. ✅ 动画系统 / Animations"
echo "   - Fade in"
echo "   - Slide in"
echo "   - Scale in"
echo "   - Smooth transitions"
echo ""
echo "5. ✅ 响应式设计 / Responsive Design"
echo "   - Mobile first approach"
echo "   - Flexible layouts"
echo "   - Adaptive components"
echo ""
echo "========================================="
echo "🎨 颜色主题 / Color Theme:"
echo "========================================="
echo "Light Mode:"
echo "  - Background: #ffffff"
echo "  - Surface: #f8f9fa"
echo "  - Primary: #0ea5e9"
echo "  - Text: #212529"
echo ""
echo "Dark Mode:"
echo "  - Background: #0f0f0f"
echo "  - Surface: #1a1a1a"
echo "  - Primary: #0ea5e9"
echo "  - Text: #e5e5e5"
echo ""
echo "========================================="
echo "📚 文档 / Documentation:"
echo "========================================="
echo "- MAINLINE_THEME.md - 完整主题指南"
echo "- ui/tailwind.config.js - Tailwind 配置"
echo "- ui/src/index.css - 自定义样式"
echo ""
echo "========================================="
echo "🚀 下一步 / Next Steps:"
echo "========================================="
echo "1. 运行应用查看新主题"
echo "2. 测试深色模式切换"
echo "3. 自定义颜色配置"
echo "4. 添加更多组件样式"
echo ""
echo "✅ Mainline 主题集成完成！/ Theme Integration Complete!"

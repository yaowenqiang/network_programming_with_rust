#!/bin/bash

echo "=== GDB 手动测试指南 ===\n"

echo "✅ GDB 安装状态: $(gdb --version | head -1)"
echo ""

echo "📝 macOS GDB 配置要求:"
echo "由于 macOS 安全限制，GDB 需要代码签名证书才能调试程序。"
echo ""

echo "🔧 基本测试 (不使用 GDB 调试权限):"
echo "1. 检查 GDB 是否可以加载程序:"
gdb --batch --ex="file simple_test" --ex="quit" 2>&1 | head -5

echo ""
echo "2. 检查 GDB 是否可以查看符号:"
gdb --batch --ex="file simple_test" --ex="info functions" --ex="quit" 2>&1 | head -10

echo ""
echo "🚀 如果你想完整使用 GDB 调试功能:"
echo "请参考 GDB_MACOS_SETUP.md 文件进行证书配置。"
echo ""

echo "📚 可用的调试程序:"
echo "- simple_test (简单测试程序)"
echo "- src/debug_example (完整调试示例)"
echo ""

echo "🎯 快速命令参考:"
echo "启动 GDB:              gdb simple_test"
echo "设置断点:              (gdb) break main"
echo "运行程序:              (gdb) run"
echo "查看变量:              (gdb) print variable"
echo "单步执行:              (gdb) next"
echo "继续执行:              (gdb) continue"
echo "退出 GDB:              (gdb) quit"
echo ""

echo "💡 提示:"
echo "- 即使没有完整调试权限，GDB 仍可用于符号查看和分析"
echo "- 对于日常 Rust 调试，也可以使用 println! 调试法"
echo "- 或者使用 Rust 的专门调试工具如 rust-gdb"
echo ""

echo "安装完成！GDB 基本功能可用。"
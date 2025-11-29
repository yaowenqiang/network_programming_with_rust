#!/bin/bash

echo "=== GDB 快速启动指南 ===\n"

echo "1. 检查调试程序..."
if [ -f "simple_test" ]; then
    echo "✅ 找到调试程序: simple_test"
    ls -la simple_test
else
    echo "❌ 未找到调试程序，重新编译..."
    rustc -g simple_test.rs -o simple_test
fi

if [ -f "src/debug_example" ]; then
    echo "✅ 找到完整示例: src/debug_example"
else
    echo "❌ 未找到完整示例，重新编译..."
    rustc -g src/debug_example.rs -o src/debug_example
fi

echo ""
echo "2. GDB 版本和状态:"
gdb --version | head -1

echo ""
echo "3. 推荐的启动方式:"
echo ""
echo "=== 方法 1: 使用 rust-gdb (最推荐) ==="
echo "命令: rust-gdb simple_test"
echo "说明: rust-gdb 是 GDB 的 Rust 包装器，提供更好的 Rust 支持"
echo ""

echo "=== 方法 2: 使用调试脚本 ==="
echo "命令: gdb -x debug_commands.gdb src/debug_example"
echo "说明: 使用预设的调试脚本，自动设置断点和配置"
echo ""

echo "=== 方法 3: 手动 GDB (基础使用) ==="
echo "命令序列:"
echo "  gdb simple_test"
echo "  (gdb) file simple_test"
echo "  (gdb) break main"
echo "  (gdb) start"
echo "  (gdb) run"
echo "  (gdb) print variable_name"
echo "  (gdb) next"
echo "  (gdb) continue"
echo "  (gdb) quit"
echo ""

echo "4. 常见错误解决:"
echo ""
echo "错误: 'Don't know how to run'"
echo "原因: GDB 没有正确加载目标程序"
echo "解决: 使用 'file' 命令加载程序，然后使用 'start' 或 'run'"
echo ""
echo "错误: 'Undefined command: break'"
echo "原因: 命令拼写错误或上下文问题"
echo "解决: 使用完整的 'break' 命令，如 'break main'"
echo ""

echo "5. 立即测试选项:"
echo ""

# 选项 1: rust-gdb 测试
echo "选项 1 - 使用 rust-gdb (推荐):"
echo "执行: rust-gdb simple_test << 'EOF'"
echo "break main"
echo "run"
echo "print \"Hello from GDB!\""
echo "continue"
echo "quit"
echo "EOF"
echo ""

# 选项 2: 使用预设脚本
echo "选项 2 - 使用调试脚本:"
echo "执行: gdb -x debug_commands.gdb src/debug_example"
echo ""

# 选项 3: 手动调试演示
echo "选项 3 - 手动调试演示:"
echo "我们可以手动演示基本的 GDB 命令序列..."

# 创建一个 GDB 自动运行脚本
cat > auto_debug.gdb << 'EOF'
# GDB 自动运行脚本
set print pretty on
set print demangle on
set language rust

echo "=== GDB 自动调试开始 ===\n"

# 加载程序
file simple_test

# 检查程序是否正确加载
echo "=== 程序信息 ===\n"
info files
echo "\n=== 函数列表 ===\n"
info functions | head -10
echo "\n=== 设置断点 ===\n"

# 在 main 函数设置断点
break main

echo "已设置断点在 main 函数\n"
echo "现在可以开始调试了...\n"

# 开始调试流程
echo "启动程序..."
start

# 等待用户交互
echo "\n=== 调试提示 ===\n"
echo "现在你可以使用以下命令:"
echo "  next          - 单步执行"
echo "  print i       - 查看变量 i 的值"
echo "  info locals   - 查看局部变量"
echo "  continue      - 继续执行到下一个断点"
echo "  quit          - 退出 GDB\n"

# 继续执行几步展示调试功能
echo "\n=== 自动演示 ===\n"

# 查看当前状态
echo "查看当前函数:"
info frame

echo "\n查看局部变量:"
info locals

# 单步执行几次
echo "\n单步执行循环:"
next
echo "第一次循环后:"
print i
next
echo "第二次循环后:"
print i

echo "\n=== 继续执行到程序结束 ===\n"
continue

echo "\n=== 调试会话结束 ===\n"
quit
EOF

echo ""
echo "选项 4 - 完全自动化调试:"
echo "执行: gdb -x auto_debug.gdb simple_test"
echo ""

echo "=== 选择你想使用的调试方式 ==="
echo ""

# 询问用户选择
echo "请选择一个选项 (1-4):"
read -p "输入选项: " choice

case $choice in
    1)
        echo "启动 rust-gdb 交互式调试..."
        rust-gdb simple_test
        ;;
    2)
        echo "使用预设调试脚本..."
        gdb -x debug_commands.gdb src/debug_example
        ;;
    3)
        echo "演示手动 GDB 命令..."
        echo "首先启动 GDB:"
        echo "gdb simple_test"
        echo ""
        echo "然后在 GDB 提示符下依次输入:"
        echo "  file simple_test"
        echo "  break main"
        echo "  start"
        echo "  print \"调试成功!\""
        echo "  continue"
        echo "  quit"
        ;;
    4)
        echo "运行自动化调试演示..."
        gdb -batch -x auto_debug.gdb simple_test
        ;;
    *)
        echo "无效选项，显示帮助信息..."
        echo ""
        echo "基本 GDB 启动命令:"
        echo "  gdb simple_test"
        echo ""
        echo "在 GDB 中:"
        echo "  (gdb) help                    # 显示帮助"
        echo "  (gdb) file program_name        # 加载程序"
        echo "  (gdb) break main               # 在 main 设置断点"
        echo "  (gdb) run                     # 运行程序"
        echo "  (gdb) start                   # 开始程序并停在 main 开始处"
        echo "  (gdb) next                    # 单步执行"
        echo "  (gdb) print variable_name      # 打印变量值"
        echo "  (gdb) continue                # 继续执行"
        echo "  (gdb) quit                    # 退出"
        ;;
esac
#!/bin/bash

# Rust GDB 调试演示脚本

echo "=== Rust GDB 调试演示 ===\n"

# 1. 检查调试文件
echo "1. 检查调试文件..."
if [ -f "src/debug_example" ]; then
    echo "✅ 调试程序已编译: src/debug_example"
    ls -la src/debug_example
else
    echo "❌ 调试程序不存在"
    echo "编译命令: rustc -g src/debug_example.rs -o src/debug_example"
    exit 1
fi

echo ""

# 2. 显示 GDB 安装检查
echo "2. GDB 安装检查..."
if command -v gdb &> /dev/null; then
    echo "✅ GDB 已安装: $(gdb --version | head -1)"
else
    echo "❌ GDB 未安装"
    echo "请安装 GDB:"
    echo "  macOS: brew install gdb"
    echo "  Ubuntu: sudo apt-get install gdb"
    echo "  CentOS: sudo yum install gdb"
fi

echo ""

# 3. 创建简化调试脚本
echo "3. 创建 GDB 调试脚本..."

cat > quick_debug.gdb << 'EOF'
# 快速调试脚本
set print pretty on
set print demangle on
set language rust

echo "=== Rust GDB 调试会话 ===\n"

# 加载程序
file src/debug_example

# 设置关键断点
break main
break fibonacci

echo "已设置断点: main, fibonacci\n"
echo "常用命令:\n"
echo "  run              - 运行程序\n"
echo "  continue         - 继续执行\n"
echo "  print var        - 打印变量\n"
echo "  info locals      - 查看局部变量\n"
echo "  backtrace        - 查看调用栈\n"
echo "  next             - 单步执行\n"
echo "  step             - 进入函数\n"
echo "  quit             - 退出\n"
EOF

echo "✅ 创建了 quick_debug.gdb 调试脚本"

echo ""

# 4. 显示调试命令示例
echo "4. GDB 调试命令示例："
echo ""

echo "=== 启动 GDB ==="
echo "方法1: gdb src/debug_example"
echo "方法2: gdb -x quick_debug.gdb src/debug_example"
echo "方法3: gdb -x debug_commands.gdb src/debug_example"
echo ""

echo "=== 基本调试流程 ==="
echo "1. 启动 GDB: gdb src/debug_example"
echo "2. 设置断点: (gdb) break main"
echo "3. 运行程序: (gdb) run"
echo "4. 查看变量: (gdb) print person1"
echo "5. 单步执行: (gdb) next"
echo "6. 继续执行: (gdb) continue"
echo ""

echo "=== 高级调试技巧 ==="
echo "条件断点: (gdb) break fibonacci if n == 3"
echo "监视变量: (gdb) watch person1.age"
echo "查看内存: (gdb) x/10x &variable"
echo "查看栈: (gdb) backtrace"
echo "线程调试: (gdb) info threads"
echo ""

# 5. 显示调试程序特性
echo "5. 调试程序特性："
echo ""
echo "包含的调试功能:"
echo "- 结构体创建和操作 (Person)"
echo "- 递归函数 (fibonacci)"
echo "- 排序算法 (bubble_sort)"
echo "- 向量处理 (process_numbers)"
echo "- 字符串操作"
echo "- 指针和引用"
echo ""

echo "可用的调试断点位置:"
echo "- main (主函数)"
echo "- Person::new (结构体构造函数)"
echo "- fibonacci (递归函数)"
echo "- bubble_sort (排序算法)"
echo "- process_numbers (向量处理)"
echo ""

# 6. 实际测试调试程序
echo "6. 测试调试程序..."
echo ""
./src/debug_example | head -10
echo "... (程序输出已截断)"
echo ""

echo "=== 文件清单 ==="
echo "创建的调试相关文件:"
echo ""
echo "程序文件:"
echo "  src/debug_example.rs      - 调试示例源代码"
echo "  src/debug_example         - 带调试信息的可执行文件"
echo "  src/debug_example.dSYM    - 调试符号信息 (macOS)"
echo ""
echo "调试脚本:"
echo "  debug_commands.gdb        - 完整调试脚本"
echo "  quick_debug.gdb           - 快速调试脚本"
echo ""
echo "文档:"
echo "  GDB_DEBUG_GUIDE.md        - 完整 GDB 调试指南"
echo ""

echo "=== 快速开始 ==="
echo "如果已安装 GDB，现在就可以开始调试:"
echo ""
echo "1. 启动调试:"
echo "   gdb -x quick_debug.gdb src/debug_example"
echo ""
echo "2. 或者使用完整脚本:"
echo "   gdb -x debug_commands.gdb src/debug_example"
echo ""
echo "3. 或者手动调试:"
echo "   gdb src/debug_example"
echo ""

echo "调试演示完成！🎯"
echo ""
echo "💡 提示: 第一次使用 GDB 调试 Rust 时，建议先从简单变量查看和单步执行开始。"
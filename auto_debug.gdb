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

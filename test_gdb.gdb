# GDB 测试脚本
set print pretty on
set language rust
set print demangle on

echo "=== GDB 测试开始 ===\n"

# 加载程序
file simple_test

# 设置断点
break main

echo "已设置断点在 main 函数\n"
echo "将运行程序并查看变量...\n"

# 运行程序
run

echo "\n=== 测试变量查看 ===\n"
echo "在断点处，你可以使用以下命令:\n"
echo "  print i           - 查看循环变量 i\n"
echo "  info locals      - 查看所有局部变量\n"
echo "  next             - 单步执行\n"
echo "  continue         - 继续执行到程序结束\n"
echo "  quit             - 退出 GDB\n"

# 自动执行一些测试
echo "正在自动执行基本测试...\n"

# 执行到循环开始
next

echo "现在可以查看循环变量 i 的值\n"

# 单步执行几次
next
print i
next
print i

echo "\n=== 测试完成 ===\n"
echo "GDB 正在继续执行到程序结束...\n"

continue

echo "\n程序执行完成！\n"
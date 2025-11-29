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

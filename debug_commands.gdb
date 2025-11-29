# GDB 调试脚本示例
# 使用方法: gdb -x debug_commands.gdb debug_example

# 设置调试环境
set print pretty on
set print static-members on
set print vtbl on
set print demangle on
set demangle-style gnu-v3
set language rust

# 加载可执行文件
file debug_example

# 在 main 函数设置断点
break main

# 在特定函数设置断点
break Person::new
break fibonacci
break bubble_sort

# 在特定行设置断点（根据行号调整）
break debug_example.rs:18
break debug_example.rs:35

# 运行程序
# run

# 如果程序已经在运行，使用以下命令
# continue

# 以下是常用的调试命令示例

# 1. 查看变量值
# print person1
# print numbers
# print &x

# 2. 查看局部变量
# info locals

# 3. 查看调用栈
# backtrace
# backtrace full

# 4. 单步执行
# next        # 单步执行（不进入函数）
# step        # 单步执行（进入函数）
# stepi       # 指令级单步
# nexti       # 指令级单步（不进入函数）

# 5. 继续执行
# continue
# finish      # 执行到当前函数返回

# 6. 条件断点
# break fibonacci if n == 3

# 7. 监视点
# watch person1.age
# info watchpoints

# 8. 查看内存
# x/10x &person1  # 查看内存内容
# x/s person1.name # 查看字符串

# 9. 查看类型信息
# ptype Person
# whatis person1

# 10. 修改变量值
# set variable person1.age = 30
# set variable numbers[0] = 100

# 调试特定函数的命令示例：
#
# 在 fibonacci 函数中：
# print n
# backtrace

# 在 bubble_sort 函数中：
# print arr
# print arr[0]
# print n

# 在 process_numbers 函数中：
# print numbers
# print num
# print result

# 线程调试命令：
# info threads
# thread 1
# thread apply all bt

# 宏定义（可选）
define print_person
    if $argc == 1
        print $arg0
        p $arg0.name
        p $arg0.age
        p $arg0.email
    else
        print "用法: print_person <Person 变量>"
    end
end

# 自动执行命令（可选）
# define my_main
#     break main
#     run
#     break fibonacci
#     continue
# end

# 保存调试会话
# save breakpoints my_breakpoints

echo "GDB 调试脚本加载完成！\n"
echo "常用命令：\n"
echo "  run             - 运行程序\n"
echo "  break           - 设置断点\n"
echo "  print           - 打印变量\n"
echo "  info locals     - 查看局部变量\n"
echo "  backtrace       - 查看调用栈\n"
echo "  next            - 单步执行\n"
echo "  continue        - 继续执行\n"
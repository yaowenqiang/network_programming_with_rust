# Rust GDB 调试完整指南

## 📋 目录

1. [环境准备](#环境准备)
2. [编译选项](#编译选项)
3. [基本 GDB 命令](#基本-gdb-命令)
4. [调试 Rust 特有功能](#调试-rust-特有功能)
5. [实际调试示例](#实际调试示例)
6. [高级调试技巧](#高级调试技巧)
7. [常见问题解决](#常见问题解决)

## 🔧 环境准备

### 安装 GDB

```bash
# macOS (需要先安装 Homebrew)
brew install gdb

# Ubuntu/Debian
sudo apt-get install gdb

# CentOS/RHEL
sudo yum install gdb

# 或者在系统包管理器中搜索 gdb
```

### 配置 GDB (macOS 特别说明)

```bash
# macOS 需要对 gdb 进行签名才能调试
# 1. 创建证书证书
# 2. 在钥匙串访问中信任证书
# 3. 重新启动终端
```

## 🛠️ 编译选项

### 1. 基本 GDB 调试编译

```bash
# 编译带调试信息的 Rust 程序
rustc -g debug_example.rs -o debug_example

# 使用 Cargo 编译调试版本
cargo build

# 运行调试版本
cargo run
```

### 2. 不同优化级别

```bash
# 无优化（最适合调试）
rustc -g -C opt-level=0 debug_example.rs -o debug_example_debug

# 默认优化级别
rustc -g debug_example.rs -o debug_example

# 最高优化级别
rustc -g -O debug_example.rs -o debug_example_optimized
```

### 3. Cargo 调试配置

```toml
# Cargo.toml
[profile.dev]
debug = true
opt-level = 0

[profile.test]
debug = true
```

```bash
# 调试构建
cargo build

# 测试调试
cargo test

# 发布版调试（不推荐，因为优化）
cargo build --release
```

## 🎯 基本 GDB 命令

### 启动和基本操作

```bash
# 启动 GDB
gdb debug_example

# 或者直接加载脚本
gdb -x debug_commands.gdb debug_example

# 在 GDB 中的基本命令
(gdb) file debug_example          # 加载可执行文件
(gdb) run                         # 运行程序
(gdb) quit                        # 退出 GDB
```

### 断点设置

```bash
# 在函数设置断点
(gdb) break main
(gdb) break Person::new
(gdb) break fibonacci

# 在行号设置断点
(gdb) break debug_example.rs:18

# 条件断点
(gdb) break fibonacci if n == 3

# 临时断点（只生效一次）
(gdb) tbreak main

# 查看所有断点
(gdb) info breakpoints

# 删除断点
(gdb) delete 1                    # 删除断点编号 1
(gdb) clear fibonacci             # 删除函数 fibonacci 的断点
```

### 程序执行控制

```bash
(gdb) run [args]                  # 运行程序（可以传递参数）
(gdb) continue                    # 继续执行
(gdb) next                        # 单步执行（不进入函数）
(gdb) step                        # 单步执行（进入函数）
(gdb) nexti                       # 指令级单步（不进入函数）
(gdb) stepi                       # 指令级单步（进入函数）
(gdb) finish                      # 执行到当前函数返回
(gdb) until                       # 执行到当前循环结束
```

### 变量和内存查看

```bash
# 查看变量值
(gdb) print person1
(gdb) print numbers[0]
(gdb) print *some_pointer

# 查看所有局部变量
(gdb) info locals

# 查看函数参数
(gdb) info args

# 查看内存内容
(gdb) x/10x &variable              # 十六进制格式显示10个字节
(gdb) x/s &string_variable         # 显示字符串
(gdb) x/20b &variable              # 二进制格式

# 查看类型信息
(gdb) whatis person1
(gdb) ptype Person

# 修改变量值
(gdb) set variable person1.age = 30
(gdb) set variable numbers[0] = 100
```

### 调用栈和线程

```bash
# 查看调用栈
(gdb) backtrace
(gdb) bt                           # backtrace 的简写

# 查看详细调用栈
(gdb) backtrace full

# 切换栈帧
(gdb) frame 2
(gdb) up                           # 向上一级栈帧
(gdb) down                         # 向下一级栈帧

# 线程调试
(gdb) info threads
(gdb) thread 1                     # 切换到线程 1
(gdb) thread apply all bt          # 查看所有线程的调用栈
```

## 🦀 调试 Rust 特有功能

### 1. 结构体调试

```bash
# 创建断点
(gdb) break Person::new

# 查看结构体
(gdb) print person1
$1 = Person {name: "张三", age: 25, email: Some("zhangsan@example.com")}

# 查看结构体字段
(gdb) print person1.name
(gdb) print person1.age

# 查看结构体的内存布局
(gdb) ptype Person
type = struct Person {
    std::string::String name,
    uint32_t age,
    std::option::Option<std::string::String> email,
}
```

### 2. 枚举和 Option 调试

```rust
// 在代码中
enum Status {
    Active,
    Inactive(String),
    Pending { id: u32 },
}

let status = Status::Inactive("用户未激活".to_string());
```

```bash
# GDB 调试
(gdb) print status
$1 = Inactive("用户未激活")

# 如果是 Some 值
(gdb) print email
$1 = Some("user@example.com")

# 访问 Option 内部的值（需要知道变体）
(gdb) print email.0     # 如果确定是 Some
(gdb) print *email     # 可能需要根据具体类型
```

### 3. Vec 和切片调试

```rust
let numbers = vec![1, 2, 3, 4, 5];
let slice = &numbers[1..4];
```

```bash
# 查看 Vec
(gdb) print numbers
$1 = Vec<i32> {len: 5, capacity: 5, buf: 0x...}

# 查看 Vec 内容
(gdb) print numbers[0]
$2 = 1
(gdb) print numbers.len()
$3 = 5usize

# 查看切片
(gdb) print slice
$4 = &[2, 3, 4]
```

### 4. 字符串调试

```rust
let string = String::from("Hello, Rust!");
let slice = &string[0..5];
```

```bash
# 查看 String
(gdb) print string
$1 = "Hello, Rust!"

# 查看字符串长度
(gdb) print string.len()
$2 = 12usize

# 查看字节表示
(gdb) print string.as_bytes()
$3 = &[72, 101, 108, 108, 111, 44, 32, 82, 117, 115, 116, 33]
```

### 5. 闭包和特征调试

```bash
# 闭包调试比较复杂，通常需要查看生成的机器码
# 使用 rustc --emit asm debug.rs 来查看生成的汇编

# 特征对象调试
(gdb) print trait_object
$1 = dyn SomeTrait = {vtable: 0x..., data: 0x...}
```

## 🔍 实际调试示例

### 示例 1: 调试排序算法

```bash
# 启动 GDB
gdb debug_example

# 在 bubble_sort 函数设置断点
(gdb) break bubble_sort

# 运行程序
(gdb) run

# 程序会在 bubble_sort 开始处暂停
# 查看输入参数
(gdb) info args
arr = (len=7, capacity=7) = [64, 34, 25, 12, 22, 11, 90]
n = 7

# 设置循环断点
(gdb) break debug_example.rs:44  # 外层循环
(gdb) continue

# 每次暂停时查看数组状态
(gdb) print arr
(gdb) print arr[j]
(gdb) print arr[j+1]

# 单步执行交换操作
(gdb) next
(gdb) print arr
```

### 示例 2: 调试递归函数

```bash
# 在 fibonacci 函数设置断点
(gdb) break fibonacci

# 设置条件断点，只在特定值时暂停
(gdb) break fibonacci if n == 3

# 运行程序
(gdb) run

# 查看调用栈
(gdb) backtrace
#0  fibonacci (n=3) at debug_example.rs:35
#1  0x000055555555... in fibonacci (n=4) at debug_example.rs:39
#2  0x000055555555... in fibonacci (n=5) at debug_example.rs:39
#3  0x000055555555... in main () at debug_example.rs:67

# 查看参数值
(gdb) print n
(gdb) info args

# 单步执行递归调用
(gdb) step
```

### 示例 3: 调试内存问题

```bash
# 创建一个会导致越界访问的程序
rustc -g memory_bug.rs -o memory_bug

# 在访问可能越界的代码处设置断点
(gdb) break memory_bug.rs:15

# 运行程序
(gdb) run

# 检查数组边界
(gdb) print arr.len()
(gdb) print index

# 在访问前检查条件
(gdb) print index < arr.len()
```

## 🚀 高级调试技巧

### 1. 监视点 (Watchpoints)

```bash
# 当变量值改变时暂停
(gdb) watch person1.age
# Hardware watchpoint 1: person1.age

# 当变量被写入时暂停
(gdb) awatch numbers[0]

# 查看所有监视点
(gdb) info watchpoints

# 删除监视点
(gdb) delete 1
```

### 2. 条件断点和命令

```bash
# 设置条件断点
(gdb) break fibonacci if n > 10

# 在断点处自动执行命令
(gdb) commands 1
> print n
> print current_stack_depth
> continue
> end
```

### 3. 自定义 GDB 命令

```gdb
# 在 .gdbinit 文件中定义自定义命令
define print_vec
    if $argc == 1
        set $vec = $arg0
        set $len = $vec.len()
        set $i = 0
        while $i < $len
            printf "vec[%d] = %d\n", $i, $vec[$i]
            set $i = $i + 1
        end
    else
        printf "用法: print_vec <vec_variable>\n"
    end
end
```

### 4. 核心转储分析

```bash
# 生成核心转储文件
ulimit -c unlimited
./debug_example  # 当程序崩溃时会生成 core 文件

# 使用 GDB 分析核心转储
gdb debug_example core

# 查看崩溃时的状态
(gdb) backtrace
(gdb) info locals
(gdb) info args
```

### 5. 远程调试

```bash
# 在远程机器上启动 gdbserver
gdbserver :1234 ./debug_example

# 在本地机器上连接
gdb debug_example
(gdb) target remote remote_ip:1234

# 然后可以正常调试
(gdb) break main
(gdb) continue
```

## ❓ 常见问题解决

### 1. 符号未找到问题

```bash
# 错误: No symbol table is loaded
# 解决: 确保使用 -g 编译
rustc -g debug_example.rs -o debug_example

# 检查调试信息
file debug_example
readelf -S debug_example | grep debug
```

### 2. Rust 特殊字符显示问题

```bash
# 设置 GDB 字符编码
set charset UTF-8
set print pretty on

# 查看 UTF-8 字符串
(gdb) set print elements 200
(gdb) print chinese_string
```

### 3. 优化代码调试困难

```bash
# 编译时禁用优化
rustc -g -C opt-level=0 debug_example.rs -o debug_example

# 或者使用 Cargo
cargo build

# 在 Cargo.toml 中设置
[profile.dev]
opt-level = 0
debug = true
```

### 4. 宏调试

```bash
# 宏展开后的代码调试
# 使用 cargo expand 查看宏展开
cargo install cargo-expand
cargo expand

# 或者在 GDB 中设置断点到宏展开后的位置
(gdb) list Macro::expanded_function
(gdb) break expanded_function
```

### 5. 异步和多线程调试

```bash
# 查看所有线程
(gdb) info threads

# 切换到特定线程
(gdb) thread 2

# 查看异步任务的栈
(gdb) backtrace

# 在 GDB 中设置 Rust 特定选项
set language rust
set print rust on
```

## 📝 GDB 配置文件

### 创建 ~/.gdbinit

```gdb
# ~/.gdbinit 文件内容

# Rust 特定设置
set print pretty on
set print static-members on
set print vtbl on
set print demangle on
set demangle-style gnu-v3
set language rust

# 自定义命令
define rust_print_str
    if $argc == 1
        printf "\"%.*s\"\n", $arg0.len, $arg0.data_ptr
    else
        printf "用法: rust_print_str <&str>\n"
    end
end

# 常用别名
alias rr = run
alias c = continue
alias n = next
alias s = step
alias bt = backtrace
```

### 项目特定调试脚本

```bash
# 创建项目的调试脚本
cat > debug_project.sh << 'EOF'
#!/bin/bash

# 编译带调试信息
cargo build

# 启动 GDB 并加载脚本
gdb -x debug_commands.gdb target/debug/your_project
EOF

chmod +x debug_project.sh
```

## 🎯 快速参考

### 常用命令速查

| 命令 | 功能 |
|------|------|
| `gdb program` | 启动 GDB |
| `break func` | 在函数 func 设置断点 |
| `break file:line` | 在文件行号设置断点 |
| `run` | 运行程序 |
| `continue` | 继续执行 |
| `next` | 单步执行（不进入函数） |
| `step` | 单步执行（进入函数） |
| `print var` | 打印变量值 |
| `info locals` | 查看局部变量 |
| `backtrace` | 查看调用栈 |
| `quit` | 退出 GDB |

### Rust 特殊调试技巧

1. **使用 `-g` 编译**: 确保包含调试信息
2. **禁用优化**: 使用 `-C opt-level=0` 便于调试
3. **查看内存**: `x/10x &variable` 查看内存内容
4. **条件断点**: `break func if condition` 设置条件断点
5. **监视点**: `watch variable` 监视变量变化

## 📚 进一步学习

- [GDB 官方文档](https://sourceware.org/gdb/documentation/)
- [Rust 调试指南](https://doc.rust-lang.org/rustc/platform-specific-docs/index.html)
- [Cargo 调试配置](https://doc.rust-lang.org/cargo/reference/profiles.html)

---

这个指南涵盖了 Rust GDB 调试的各个方面，从基础使用到高级技巧。记得多练习，调试技能会随着经验增长！
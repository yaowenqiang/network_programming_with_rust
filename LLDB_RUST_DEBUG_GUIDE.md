# LLDB 调试 Rust 程序完整指南

## 📋 目录

1. [LLDB 简介](#lldb-简介)
2. [LLDB vs GDB 对比](#lldb-vs-gdb-对比)
3. [编译 LLDB 调试版本](#编译-lldb-调试版本)
4. [基本 LLDB 命令](#基本-lldb-命令)
5. [LLDB 调试示例](#lldb-调试示例)
6. [高级 LLDB 技术](#高级-lldb-技术)
7. [Rust 特定调试](#rust-特定调试)
8. [LLDB 配置和优化](#lldb-配置和优化)
9. [常见问题解决](#常见问题解决)
10. [最佳实践](#最佳实践)

---

## LLDB 简介

### 什么是 LLDB？

LLDB 是 LLVM 项目的一部分，是 macOS 和其他基于 LLVM 系统的默认调试器。相比 GDB，LLDB 在 Rust 调试中有很多优势：

**优势**:
- ✅ **macOS 原生支持** - 无需额外配置权限
- ✅ **更好的 Rust 支持** - 现代 IDE 集成
- ✅ **更快的启动速度** - 优化的启动流程
- ✅ **丰富的 Python API** - 可扩展的调试脚本
- ✅ **更好的多线程支持** - 现代线程调试
- ✅ **集成的内存检查** - 内置的内存安全工具

### 为什么选择 LLDB 调试 Rust？

1. **macOS 兼容性最佳** - 在 macOS 上无需额外配置
2. **Rust 特定的美化输出** - 更好的类型显示
3. **现代调试器特性** - 支持 Rust 的所有权系统
4. **IDE 集成** - VS Code、Xcode 等原生支持

---

## LLDB vs GDB 对比

| 特性 | LLDB | GDB |
|------|------|------|
| **macOS 支持** | ✅ 原生支持，无需配置 | ⚠️ 需要代码签名证书 |
| **启动速度** | ⚡ 更快 | 🐌 相对较慢 |
| **Rust 支持** | ✅ 现代，持续更新 | ✅ 成熟，但需要 rust-gdb |
| **多线程调试** | ✅ 优秀 | ✅ 良好 |
| **Python 扩展** | ✅ 丰富的 API | ✅ 成熟的 API |
| **IDE 集成** | ✅ 原生支持 | ✅ 广泛支持 |
| **学习曲线** | 📈 相对平缓 | 📈 相对陡峭 |
| **权限要求** | ✅ 无特殊要求 | ⚠️ 需要 root 或证书 |

### 推荐场景

**选择 LLDB 的场景**:
- 在 macOS 上开发 Rust
- 需要快速设置和调试
- 使用现代 IDE (VS Code、RustRover)
- 需要多线程调试
- 偏爱 Python 脚本化

**选择 GDB 的场景**:
- 需要跨平台一致性
- 已熟悉 GDB 命令
- 需要特定的 GDB 插件
- 在 Linux 上开发为主

---

## 编译 LLDB 调试版本

### 1. 基本编译

```bash
# 编译带调试信息的程序
rustc -g your_program.rs -o your_program

# 使用 Cargo 编译
cargo build

# 禁用优化（推荐用于调试）
rustc -g -C opt-level=0 your_program.rs -o your_program

# Cargo 禁用优化
cargo build
```

### 2. 高级编译选项

```bash
# 启用所有调试信息
rustc -g -C debuginfo=2 your_program.rs -o your_program

# 禁用内联优化
rustc -g -C no-inline your_program.rs -o your_program

# 保持堆栈帧
rustc -g -C force-frame-pointers=yes your_program.rs -o your_program

# Cargo 完整调试配置
RUSTFLAGS="-C debuginfo=2 -C no-inline" cargo build
```

### 3. 检查调试信息

```bash
# 查看可执行文件的调试信息
dwarfdump your_program

# 检查符号表
nm your_program | grep -E "main|User"

# 查看程序头信息
file your_program
```

---

## 基本 LLDB 命令

### 启动和退出

```bash
# 启动 LLDB 调试会话
lldb program_name

# 启动并传递参数
lldb -- program_name arg1 arg2

# 批处理模式
lldb -b --one-line "command1; command2" program_name

# 退出 LLDB
(lldb) quit
(lldb) q

# 清除当前会话
(lldb) session save
(lldb) session load
```

### 程序控制

```bash
# 设置断点
(lldb) breakpoint set --name main
(lldb) br set -n main

# 在指定行设置断点
(lldb) breakpoint set --file main.rs --line 42
(lldb) br s -f main.rs -l 42

# 条件断点
(lldb) breakpoint set --condition "x == 42" --name main

# 运行程序
(lldb) run
(lldb) r

# 继续执行
(lldb) continue
(lldb) c

# 单步执行
(lldb) next
(lldb) n

# 进入函数
(lldb) step
(lldb) s

# 单步执行指令
(lldb) next-instruction
(lldb) ni

# 进入指令
(lldb) step-instruction
(lldb) si

# 退出当前函数
(lldb) finish
(lldb) fin
```

### 变量查看

```bash
# 查看变量值
(lldb) frame variable variable_name
(lldb) fr v variable_name
(lldb) v variable_name

# 查看所有局部变量
(lldb) frame variable
(lldb) fr v
(lldb) v

# 查看变量类型
(lldb) frame variable --show-types variable_name
(lldb) v -t variable_name

# 查看内存
(lldb) memory read --size 4 --format x --count 10 &variable_name
(lldb) x -s 4 -f x -c 10 &variable_name

# 查看寄存器
(lldb) register read
(lldb) re r

# 查看特定寄存器
(lldb) register read rsp
(lldb) re r rsp
```

### 调用栈

```bash
# 查看调用栈
(lldb) thread backtrace
(lldb) bt

# 查看完整调用栈信息
(lldb) thread backtrace --all
(lldb) bt all

# 切换栈帧
(lldb) frame select 2
(lldb) fr s 2
(lldb) f 2

# 向上移动栈帧
(lldb) frame select --relative +1
(lldb) fr s -r +1
(lldb) up

# 向下移动栈帧
(lldb) frame select --relative -1
(lldb) fr s -r -1
(lldb) down

# 返回到特定帧
(lldb) frame select 0
(lldb) fr s 0
```

### 断点管理

```bash
# 列出所有断点
(lldb) breakpoint list
(lldb) br l

# 删除断点
(lldb) breakpoint delete 1
(lldb) br del 1

# 禁用断点
(lldb) breakpoint disable 1
(lldb) br dis 1

# 启用断点
(lldb) breakpoint enable 1
(lldb) br en 1

# 临时断点（只生效一次）
(lldb) breakpoint set --name main --one-shot
(lldb) br s -n main -o

# 监视点
(lldb) watchpoint set variable variable_name
(lldb) w s v variable_name

# 条件监视点
(lldb) watchpoint set variable variable_name --condition "variable_name > 100"
(lldb) w s v variable_name -c "variable_name > 100"
```

### 搜索和导航

```bash
# 在源代码中搜索
(lldb) image lookup --name function_name
(lldb) im loo -n function_name

# 查找函数
(lldb) image lookup --regex "pattern"
(lldb) im loo -r "pattern"

# 列出当前源文件
(lldb) source list

# 显示指定行数的源代码
(lldb) source list --count 20 --line 100
(lldb) s l -c 20 -l 100

# 显示当前执行行的源代码
(lldb) source list
(lldb) s l
```

---

## LLDB 调试示例

### 示例 1: 调试基本程序

```bash
# 启动 LLDB
lldb src/lldb_example

# 在 main 函数设置断点
(lldb) breakpoint set --name main
Breakpoint 1: where = lldb_example`main + 25 at lldb_example.rs:241

# 运行程序
(lldb) run
=== Rust LLDB 调试示例程序 ===

1. 结构体调试:
创建用户: ID=1, 姓名=张三, 年龄=25

# 查看局部变量
(lldb) frame variable
(User) user1 = {
  id = 1
  name = "张三"
  age = 25
  email = None
  active = true
  scores = size=0 {}
  metadata = size=2 {
    [0] = {
      key = "role"
      value = "user"
    }
    [1] = {
      key = "department"
      value = "engineering"
    }
  }
}

# 单步执行
(lldb) next
process exited with status = 0 (0x00000000)
```

### 示例 2: 调试结构体

```bash
# 重新启动
lldb src/lldb_example
(lldb) br s -n main
(lldb) r

# 执行到结构体创建
(lldb) next
(lldb) next
用户 1 符合升级条件

# 查看结构体字段
(lldb) frame variable user1.name
(String) user1.name = "张三"

(lldb) frame variable user1.age
(u32) user1.age = 25

(lldb) frame variable user1.scores
(Vec<i32>) user1.scores = size=0 {}

# 查看结构体的内存布局
(lldb) memory read --size 4 --format x &user1
0x7ff7bfeff520: 0x00000001
0x7ff7bfeff524: 0x00000019
```

### 示例 3: 调试递归函数

```bash
# 在 fibonacci 函数设置断点
(lldb) breakpoint set --name fibonacci_debug

# 运行到递归部分
(lldb) run
(lldb) c
(lldb) c
(lldb) c

# 查看调用栈
(lldb) thread backtrace
* thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 1.1
  * frame #0: 0x0000000100003d10 lldb_example`fibonacci_debug(u32) at lldb_example.rs:58
    frame #1: 0x0000000100003dec lldb_example`fibonacci_debug(u32) at lldb_example.rs:64
    frame #2: 0x0000000100003dec lldb_example`fibonacci_debug(u32) at lldb_example.rs:64
    frame #3: 0x0000000100003dec lldb_example`fibonacci_debug(u32) at lldb_example.rs:64
    frame #4: 0x0000000100003ec4 lldb_example`main at lldb_example.rs:251

# 查看不同栈帧的参数
(lldb) frame select 1
(lldb) frame variable n
(u32) n = 5

(lldb) frame select 2
(lldb) frame variable n
(u32) n = 4

(lldb) frame select 0
(lldb) frame variable n
(u32) n = 3
```

### 示例 4: 调试向量和切片

```bash
# 在 process_data 函数设置断点
(lldb) breakpoint set --name process_data
(lldb) run

# 执行到数据处理
(lldb) c
处理数据: 输入 = [3, 8, 2, 7, 4, 9, 1]

# 查看向量参数
(lldb) frame variable numbers
(&[i32]) numbers = 0x00007ff7bfeff420

# 查看向量内容
(lldb) memory read --size 4 --format d --count 7 0x00007ff7bfeff420
0x00007ff7bfeff420: [3] [8] [2] [7] [4] [9] [1]

# 单步执行查看处理过程
(lldb) next
处理索引 0, 数值 3

(lldb) frame variable num
(&i32) num = 0x00007ff7bfeff3c

# 解引用指针
(lldb) frame variable *num
(i32) *num = 3

# 继续单步执行
(lldb) next
  3 是奇数，3x+1 -> 10

(lldb) next
  添加到结果: 10
```

---

## 高级 LLDB 技术

### 1. 自定义调试脚本

创建 `~/.lldbinit` 文件来配置 LLDB：

```python
# ~/.lldbinit - LLDB Python 初始化脚本

def rust_debug_help():
    print("""
=== Rust LLDB 调试快捷命令 ===
frame variable - 显示当前帧变量
thread backtrace - 显示调用栈
breakpoint set -n function_name - 设置断点
continue - 继续执行
next - 单步执行
step - 进入函数
finish - 退出当前函数

=== Rust 特定命令 ===
type_summary add - 简化类型显示
script rust_help() - 显示此帮助信息
""")

def rust_vec_info(vec):
    """查看 Rust Vec 的详细信息"""
    print(f"Vec 信息:")
    print(f"  指针: 0x{vec.GetChildMemberWithName('ptr').GetValueAsUnsigned():x}")
    print(f"  长度: {vec.GetChildMemberWithName('len').GetValueAsUnsigned()}")
    print(f"  容量: {vec.GetChildMemberWithName('capacity').GetValueAsUnsigned()}")

def rust_string_info(s):
    """查看 Rust String 的详细信息"""
    print(f"String 信息:")
    print(f"  指针: 0x{s.GetChildMemberWithName('data_ptr').GetValueAsUnsigned():x}")
    print(f"  长度: {s.GetChildMemberWithName('length').GetValueAsUnsigned()}")

# 注册命令
def __lldb_init_module(debugger):
    debugger.HandleCommand('command script add -f rust_help rust_help')
    debugger.HandleCommand('command script add -f rust_vec_info rust_vec')
    debugger.HandleCommand('command script add -f rust_string_info rust_str')
```

### 2. 条件调试

```bash
# 在满足特定条件时停止
(lldb) breakpoint set --name main --condition "user_count > 10"

# 设置复杂的条件表达式
(lldb) breakpoint set --name process_data --condition "index == 3 && *num > 5"

# 只在特定线程时停止
(lldb) breakpoint set --name main --thread 2

# 在特定模块中停止
(lldb) breakpoint set --name "my_module::function_name"
```

### 3. 监视点调试

```bash
# 监视变量变化
(lldb) watchpoint set variable user_count
Watchpoint created: Watchpoint 1: addr = 0x7ff7bfeff448, size = 4, state = enabled

# 监视内存地址
(lldb) watchpoint set expression -- &some_vector

# 条件监视
(lldb) watchpoint set variable some_value --condition "some_value > 100"

# 查看监视点历史
(lldb) watchpoint list
```

### 4. 内存调试

```bash
# 查看特定范围的内存
(lldb) memory read --size 8 --format x 0x100000000 --count 16

# 查看字符串内容
(lldb) memory read --size 1 --format c --count 100 0x100001000

# 反汇编函数
(lldb) disassemble --name main
(lldb) disassemble --start-address 0x100000000 --end-address 0x100000100

# 查看内存映射
(lldb) image list
```

---

## Rust 特定调试

### 1. 调试所有权和借用

```rust
fn ownership_example() {
    let s1 = String::from("Hello");
    let s2 = s1; // s1 的所有权移动到 s2
    // println!("{}", s1); // 这会报错：value borrowed here after move
    println!("{}", s2);
}
```

```bash
# 在所有权移动后查看变量
(lldb) frame variable s1
error: couldn't resolve the symbol 's1'

(lldb) frame variable s2
(String) s2 = "Hello"
```

### 2. 调试 Option 和 Result

```rust
fn option_example() {
    let maybe_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    match maybe_value {
        Some(v) => println!("Some: {}", v),
        None => println!("None"),
    }
}
```

```bash
# 查看 Option 的变体
(lldb) frame variable maybe_value
(std::option::Option<i32>) maybe_value = Some {
  __0 = 42
}

# 检查 Option 是否为 Some
(lldb) expression maybe_value.is_some()
(bool) $0 = true

(lldb) expression maybe_value.unwrap()
(i32) $1 = 42
```

### 3. 调试生命周期

```rust
fn lifetime_example<'a>(data: &'a str) -> &'a str {
    if data.len() > 10 {
        &data[0..10]
    } else {
        data
    }
}
```

```bash
# 查看借用检查器信息（需要编译时参数）
RUSTFLAGS="-Z borrowck=mem" cargo build

# 查看引用
(lldb) frame variable data
(&str) data = "Hello, world!"

# 查看引用的内容
(lldb) expression *data
error: expression failed to parse
```

### 4. 调试闭包和迭代器

```rust
fn closure_example() {
    let numbers = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = numbers.iter()
        .map(|x| x * 2)
        .filter(|&x| x > 5)
        .collect();
}
```

```bash
# 设置断点在闭包内部
(lldb) breakpoint set --file main.rs --line 300
(lldb) run

# 查看闭包捕获的变量
(lldb) frame variable x
(i32) x = 2

# 查看迭代器状态（较复杂，可能需要内存分析）
(lldb) memory read --size 8 --format x 0x7ff7bfeff400
```

---

## LLDB 配置和优化

### 1. 环境变量

```bash
# 设置 Rust 特定的环境变量
export RUST_LOG=debug          # 启用调试日志
export RUST_BACKTRACE=1       # 显示调用栈
export RUST_NIGHTLY=1        # 使用夜间编译器特性

# LLDB 特定设置
export LLDB_DEBUG_FILE=lldb_debug.log
export LLDB_DEBUG_CATEGORY=lldb
```

### 2. LLDB 配置文件

```bash
# ~/.lldbinit 文件内容
settings set target.inline-breakpoint-strategy always
settings set stop-disassembly-count 20
settings set frame-format short

# 设置默认显示选项
settings set stop-show-target true
settings set stop-show-source true

# 启用类型美化
type summary add std::string::String -s "${var._M_dataplus._M_p._M_data}"
type summary add std::vector::vector -s "${var.__begin_[0]}[size=${var.__end_[0]}-var.__begin_[0]}"
```

### 3. 性能优化设置

```bash
# 禁用符号加载优化
settings set target.prefer-dynamic-value no-dynamic-values

# 加快启动速度
settings set target.load-cwd-lldbinit false

# 优化内存读取
settings set target.x86-disassembly-flavor intel
```

---

## 常见问题解决

### 问题 1: 符号未找到

**错误**: `error: unable to resolve variable`

**解决方案**:
```bash
# 确保使用 -g 编译
rustc -g your_program.rs -o your_program

# 检查调试信息
dwarfdump your_program | head -10

# 强制重新编译
cargo clean && cargo build
```

### 问题 2: 断点不生效

**错误**: 断点设置但程序没有停止

**解决方案**:
```bash
# 检查断点状态
(lldb) breakpoint list

# 确保函数名正确
(lldb) image lookup --name function_name

# 使用文件行号设置断点
(lldb) breakpoint set --file main.rs --line 42

# 检查优化级别（可能影响断点）
settings set target.process.stop-on-exec false
```

### 问题 3: 类型显示不清晰

**错误**: 复杂类型显示混乱

**解决方案**:
```bash
# 使用 type summary 命令
(lldb) type summary add MyStruct --summary-string "${var.field1}, ${var.field2}"

# 查看内存布局
(lldb) memory read --size 4 --format x &variable_name

# 使用表达式求值
(lldb) expression variable_name.method()
```

### 问题 4: 程序退出太快

**错误**: 程序在断点前就退出

**解决方案**:
```bash
# 在 main 开始设置断点
(lldb) breakpoint set --name main

# 使用条件断点
(lldb) breakpoint set --name main --condition "argc > 1"

# 设置程序参数
(lldb) settings set -- target.run-args arg1 arg2
```

### 问题 5: 多线程调试问题

**错误**: 无法查看其他线程状态

**解决方案**:
```bash
# 列出所有线程
(lldb) thread list

# 切换到特定线程
(lldb) thread select 2

# 查看所有线程的调用栈
(lldb) thread backtrace --all
```

---

## 最佳实践

### 1. 编译优化

```toml
# Cargo.toml
[profile.dev]
debug = 2          # 最大调试信息
opt-level = 0       # 禁用优化
overflow-checks = true # 启用溢出检查
debug-assertions = true # 启用调试断言

[profile.test]
debug = 2
opt-level = 0
```

### 2. 调试友好的代码

```rust
// 使用 Debug trait
#[derive(Debug)]
struct MyStruct {
    field1: i32,
    field2: String,
}

impl MyStruct {
    fn new(value: i32, name: String) -> Self {
        let result = MyStruct {
            field1: value,
            field2: name,
        };

        // 调试输出
        println!("创建 MyStruct: {:?}", result);
        result
    }
}

fn main() {
    // 使用 assert! 进行早期验证
    let value = get_value();
    assert!(value >= 0, "value 必须非负: {}", value);

    // 使用 dbg! 宏 (Rust 1.32+)
    let result = process(value);
    dbg!(&result); // 显示位置和值

    result
}
```

### 3. 调试工作流

1. **问题复现**:
   ```bash
   # 使用明确的测试用例
   cargo test specific_test_case
   ```

2. **初步诊断**:
   ```bash
   # 使用 println! 和 dbg! 宏
   println!("DEBUG: 处理值 = {}", value);
   let result = dbg!(calculate(value));
   ```

3. **深度调试**:
   ```bash
   # 启动 LLDB 会话
   lldb target_program
   (lldb) br s -n problem_function
   (lldb) r
   (lldb) fr v
   ```

4. **问题修复**:
   ```bash
   # 修复后重新测试
   cargo test
   ```

### 4. 性能调试

```rust
// 使用条件编译来添加性能监控
#[cfg(debug_assertions)]
fn debug_time<T, F>(name: &str, f: F) -> T
where
    F: FnOnce() -> T
{
    let start = std::time::Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("DEBUG[{}]: {:?}", name, duration);
    result
}

fn main() {
    debug_time("main_logic", || {
        // 你的主要逻辑
    });
}
```

---

## 🚀 快速开始

### 立即调试我们的示例：

```bash
# 1. 编译示例程序
rustc -g src/lldb_example.rs -o src/lldb_example

# 2. 启动 LLDB 调试
lldb src/lldb_example

# 3. 在 main 函数设置断点
(lldb) breakpoint set --name main

# 4. 运行程序
(lldb) run

# 5. 开始调试！
(lldb) frame variable
(lldb) next
(lldb) continue
```

### 常用调试命令速查：

| 命令 | 简写 | 功能 |
|------|------|------|
| `frame variable` | `fr v` | 查看当前帧变量 |
| `thread backtrace` | `bt` | 查看调用栈 |
| `breakpoint set --name` | `br s -n` | 设置函数断点 |
| `continue` | `c` | 继续执行 |
| `next` | `n` | 单步执行（不进入函数）|
| `step` | `s` | 单步执行（进入函数）|
| `finish` | `fin` | 退出当前函数 |
| `quit` | `q` | 退出 LLDB |

---

## 📚 延伸阅读

- [LLDB 官方文档](https://lldb.llvm.org/)
- [Rust 调试指南](https://doc.rust-lang.org/rustc/platform-specific-docs/index.html)
- [Cargo 构建配置](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [Rust 性能分析](https://doc.rust-lang.org/book/ch14-03-cargo.html#publishing-to-cratesio)

---

## 🎯 总结

LLDB 为 Rust 调试提供了强大而现代的工具集：

✅ **macOS 最佳支持** - 无需额外配置
✅ **丰富的调试功能** - 断点、监视、内存查看
✅ **Rust 友好** - 更好的类型显示和所有权支持
✅ **扩展性强** - Python API 和自定义脚本
✅ **IDE 集成** - VS Code、Xcode 原生支持

通过本指南，你应该能够：
- 掌握 LLDB 基本和高级命令
- 调试各种 Rust 数据结构
- 解决常见的调试问题
- 优化调试工作流程

**开始调试你的 Rust 程序吧！** 🚀
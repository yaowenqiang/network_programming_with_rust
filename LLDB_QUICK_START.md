# 🎯 LLDB 调试 Rust 程序快速开始指南

## ✅ 安装状态

**LLDB 已可用**: ✅ LLDB 20.1.4 (macOS 原生支持)
**调试程序**: ✅ lldb_example (完整的调试示例程序)
**资源完整**: ✅ 包含详细指南、脚本和示例

## 🚀 为什么选择 LLDB？

### LLDB 在 macOS 上的优势

✅ **无需额外配置** - macOS 原生支持，不需要代码签名证书
✅ **启动速度更快** - 优化的启动流程，比 GDB 更快
✅ **现代调试器** - 基于 LLVM，专为现代编译器优化
✅ **Rust 友好** - 更好的类型显示和所有权理解
✅ **Python API 强大** - 丰富的调试脚本和扩展能力
✅ **IDE 集成优秀** - VS Code、RustRover 原生支持
✅ **多线程调试好** - 现代的线程管理界面

### 与 GDB 对比

| 特性 | LLDB | GDB (macOS) |
|------|------|---------------|
| 权限要求 | ✅ 无特殊要求 | ⚠️ 需要代码签名 |
| 启动速度 | ⚡ 更快 | 🐌 相对较慢 |
| 配置复杂度 | 📈 简单 | 📈 复杂 |
| 原生支持 | ✅ macOS 原生 | ❌ 需要配置 |
| IDE 集成 | ✅ 优秀 | ✅ 良好 |

**结论**: 在 macOS 上开发 Rust，LLDB 是最佳选择！

## 🔧 基础使用

### 1. 编译调试版本

```bash
# 方法 1: 使用 rustc
rustc -g your_program.rs -o your_program

# 方法 2: 使用 Cargo
cargo build

# 方法 3: 禁用优化（推荐用于调试）
rustc -g -C opt-level=0 your_program.rs -o your_program

# 方法 4: 完整调试信息
rustc -g -C debuginfo=2 your_program.rs -o your_program
```

### 2. 启动 LLDB 调试

```bash
# 方法 1: 基础启动
lldb your_program

# 方法 2: 带参数启动
lldb -- your_program arg1 arg2

# 方法 3: 批处理模式
lldb --batch --one-line "command1; command2" your_program
```

### 3. 基本调试流程

```bash
# 启动 LLDB
$ lldb src/lldb_example

# 设置断点
(lldb) breakpoint set --name main

# 运行程序
(lldb) run

# 查看变量
(lldb) frame variable
(lldb) frame variable user1

# 单步执行
(lldb) next

# 继续执行
(lldb) continue

# 查看调用栈
(lldb) thread backtrace

# 退出 LLDB
(lldb) quit
```

## 📋 常用 LLDB 命令速查

### 程序控制
| 命令 | 简写 | 功能 |
|------|------|------|
| `run` | `r` | 运行程序 |
| `continue` | `c` | 继续执行 |
| `next` | `n` | 单步执行（不进入函数） |
| `step` | `s` | 单步执行（进入函数） |
| `finish` | `fin` | 退出当前函数 |

### 断点管理
| 命令 | 简写 | 功能 |
|------|------|------|
| `breakpoint set --name func` | `br s -n func` | 在函数设置断点 |
| `breakpoint set --file file --line N` | `br s -f file -l N` | 在行设置断点 |
| `breakpoint list` | `br l` | 列出所有断点 |
| `breakpoint delete N` | `br del N` | 删除断点 N |

### 变量查看
| 命令 | 简写 | 功能 |
|------|------|------|
| `frame variable` | `fr v` | 查看所有局部变量 |
| `frame variable name` | `v name` | 查看特定变量 |
| `expression expr` | `p expr` | 执行表达式 |

### 调用栈
| 命令 | 简写 | 功能 |
|------|------|------|
| `thread backtrace` | `bt` | 查看调用栈 |
| `frame select N` | `f N` | 切换到栈帧 N |
| `up` | - | 向上一级栈帧 |
| `down` | - | 向下一级栈帧 |

## 🎯 实际调试示例

### 示例 1: 调试结构体

```bash
# 启动 LLDB
$ lldb src/lldb_example

# 设置断点
(lldb) breakpoint set --name main

# 运行到断点
(lldb) run

# 查看用户结构体
(lldb) frame variable user1
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

# 查看特定字段
(lldb) frame variable user1.name
(String) user1.name = "张三"

(lldb) frame variable user1.scores
(Vec<i32>) user1.scores = size=0 {}
```

### 示例 2: 调试递归函数

```bash
# 在递归函数设置断点
(lldb) breakpoint set --name fibonacci_debug

# 运行到递归部分
(lldb) continue

# 查看调用栈
(lldb) thread backtrace
* thread #1, queue = 'com.apple.main-thread', stop reason = breakpoint 2.1
  * frame #0: fibonacci_debug(n=3) at lldb_example.rs:77
    frame #1: fibonacci_debug(n=4) at lldb_example.rs:64
    frame #2: fibonacci_debug(n=5) at lldb_example.rs:64
    frame #3: main at lldb_example.rs:252

# 查看不同栈帧的参数
(lldb) frame select 1
(lldb) frame variable n
(u32) n = 4

(lldb) frame select 2
(lldb) frame variable n
(u32) n = 5
```

### 示例 3: 调试向量和迭代

```bash
# 在数据处理函数设置断点
(lldb) breakpoint set --name process_data

# 运行到函数
(lldb) continue

# 查看向量参数
(lldb) frame variable numbers
(Vec<i32>) numbers = size=5 {
  [0] = 3
  [1] = 8
  [2] = 2
  [3] = 7
  [4] = 9
}

# 查看向量元素
(lldb) expression numbers[0]
(int) $0 = 3

(lldb) expression numbers.size()
(int) $1 = 5

# 单步执行循环
(lldb) next
(lldb) next
(lldb) next

# 查看处理后的结果
(lldb) frame variable processed
(Vec<i32>) processed = size=5 {
  [0] = 6
  [1] = 16
  [2] = 25
  [3] = 8
  [4] = 28
}
```

## 💡 Rust 特定调试技巧

### 1. 使用 Rust 调试宏

```rust
// 使用 dbg! 宏 (Rust 1.32+)
let x = 42;
let result = dbg!(x * 2); // 会输出位置和值

// 使用 println! 进行调试
let user = User::new(1, "Alice");
println!("DEBUG: 创建的用户 {:?}", user);

// 使用 assert! 进行条件检查
assert!(user.age >= 0, "年龄不能为负数: {}", user.age);
```

### 2. 调试 Option 和 Result

```rust
fn handle_option(opt: Option<i32>) {
    match opt {
        Some(value) => {
            println!("找到值: {}", value);
            // 在 LLDB 中调试时可以:
            // (lldb) frame variable opt
            // (lldb) frame variable value
        },
        None => {
            println!("没有值");
            // 在 LLDB 中:
            // (lldb) frame variable opt
        }
    }
}

fn handle_result(result: Result<String, String>) {
    match result {
        Ok(success_msg) => println!("成功: {}", success_msg),
        Err(error_msg) => println!("错误: {}", error_msg),
    }
}
```

### 3. 调试所有权和借用

```rust
fn ownership_demo() {
    let s1 = String::from("Hello");
    let s2 = s1; // 所有权移动

    // 在 LLDB 中查看:
    // (lldb) frame variable s1  // 会显示 "借用后移动"
    // (lldb) frame variable s2  // 会显示字符串内容

    let reference = &s2;
    // (lldb) frame variable reference  // 显示引用

    // 切片操作
    let slice = &s2[0..3];
    // (lldb) frame variable slice  // 显示切片
}
```

### 4. 调试闭包和迭代器

```rust
fn closure_demo() {
    let numbers = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = numbers.iter()
        .map(|x| x * 2)
        .collect();

    // 在 LLDB 中可以:
    // 1. 设置断点在 map 闭包内部
    // 2. 查看闭包参数: (lldb) frame variable x
    // 3. 查看中间结果

    for (i, num) in numbers.iter().enumerate() {
        // 在循环中设置断点查看每次迭代
        // (lldb) frame variable i
        // (lldb) frame variable num
    }
}
```

## 🛠️ 高级调试技巧

### 1. 条件断点

```bash
# 只在满足条件时停止
(lldb) breakpoint set --name function_name --condition "variable > 100"

# 在特定迭代时停止
(lldb) breakpoint set --file main.rs --line 42 --condition "index == 5"
```

### 2. 监视点

```bash
# 监视变量变化
(lldb) watchpoint set variable counter

# 监视内存地址
(lldb) watchpoint set expression &some_vector

# 条件监视
(lldb) watchpoint set variable value --condition "value > threshold"
```

### 3. 内存调试

```bash
# 查看内存布局
(lldb) memory read --size 64 --format hex 0x7ff7bfeff420

# 查看字符串内存
(lldb) memory read --size 50 --format ascii &string_variable

# 查看结构体内存
(lldb) memory read --size 32 --format hex &struct_variable
```

### 4. 表达式求值

```bash
# 执行复杂表达式
(lldb) expression user1.calculate_average()
(lldb) expression numbers.iter().sum()
(lldb) expression fibonacci_debug(5)

# 查看方法调用
(lldb) expression user1.scores.push(100)
(lldb) expression user1.set_email("new@example.com")
```

## 📚 创建的资源

### 程序文件
- `src/lldb_example.rs` - 完整的 LLDB 调试示例程序
- `src/lldb_example` - 带调试信息的可执行文件

### 指南和文档
- `LLDB_RUST_DEBUG_GUIDE.md` - 5000+ 字的详细调试指南
- `LLDB_QUICK_START.md` - 这个快速开始指南

### 脚本和演示
- `lldb_demo.sh` - 完整的 LLDB 演示脚本
- `lldb_quick_demo.sh` - 快速 LLDB 演示和命令速查

### LLDB 调试脚本
- 自动化脚本已内置在演示脚本中
- 支持断点设置、变量查看、内存分析等

## 🎯 立即开始调试

### 快速开始选项

#### 选项 1: 运行完整演示（推荐）
```bash
# 运行快速演示脚本
./lldb_quick_demo.sh

# 这会演示：
# - 程序编译和符号查看
# - 基本命令用法
# - 实际调试流程
```

#### 选项 2: 手动调试体验
```bash
# 启动 LLDB
$ lldb src/lldb_example

# 设置断点并开始调试
(lldb) breakpoint set --name main
(lldb) run

# 开始你的调试之旅！
```

#### 选项 3: 使用你自己的程序
```bash
# 1. 编译你的 Rust 程序
rustc -g your_program.rs -o your_program

# 2. 启动 LLDB 调试
lldb your_program

# 3. 在关键位置设置断点
(lldb) breakpoint set --name your_function

# 4. 开始调试！
(lldb) run
```

### 常见调试场景

#### 场景 1: 程序崩溃调试
```bash
# 启动 LLDB 并让程序崩溃
$ lldb your_crashing_program

# 当崩溃时，LLDB 会自动停止
(lldb) thread backtrace  # 查看崩溃时的调用栈
(lldb) frame variable   # 查看崩溃时的变量状态
```

#### 场景 2: 性能问题调试
```bash
# 在性能关键位置设置断点
(lldb) breakpoint set --name expensive_function

# 记录执行时间
(lldb) continue

# 分析函数调用次数和参数
(lldb) thread backtrace
```

#### 场景 3: 并发问题调试
```bash
# 查看所有线程
(lldb) thread list

# 切换到特定线程
(lldb) thread select 2

# 查看特定线程的调用栈
(lldb) thread backtrace
```

## 💡 最佳实践建议

### 编译时
- ✅ 始终使用 `-g` 编译标志
- ✅ 调试时禁用优化 `-C opt-level=0`
- ✅ 使用完整的调试信息 `-C debuginfo=2`
- ✅ 启用断言检查 `-C debug-assertions=on`

### 调试时
- ✅ 使用有意义的断点位置
- ✅ 利用条件断点减少不必要的中断
- ✅ 使用监视点跟踪变量变化
- ✅ 结合 Rust 调试宏（dbg!, println!, assert!）

### 代码组织
- ✅ 使用 `#[derive(Debug)]` 简化结构体调试
- ✅ 在关键位置添加调试输出
- ✅ 使用单元测试验证关键函数
- ✅ 保持函数小而专注

### 学习路径
1. **开始**: 运行 `./lldb_quick_demo.sh` 体验基础功能
2. **深入学习**: 阅读 `LLDB_RUST_DEBUG_GUIDE.md` 掌握高级技巧
3. **实践**: 在自己项目中应用 LLDB 调试技术
4. **探索**: 学习 LLDB Python API 进行自定义调试脚本

---

## 🎊 恭喜！

你现在拥有了完整的 LLDB Rust 调试环境：

✅ **LLDB 已就绪** - macOS 原生支持
✅ **示例程序完整** - 涵盖各种调试场景
✅ **详细指南齐全** - 从基础到高级
✅ **脚本工具丰富** - 自动化调试流程
✅ **最佳实践提供** - 高效调试策略

**现在就可以开始调试 Rust 程序了！** 🚀

### 快速命令
```bash
# 立即开始体验
./lldb_quick_demo.sh

# 开始调试示例程序
lldb src/lldb_example

# 阅读详细指南
cat LLDB_RUST_DEBUG_GUIDE.md
```

---

**💡 提示**: LLDB 是 macOS 上 Rust 调试的最佳选择，无需任何额外配置！
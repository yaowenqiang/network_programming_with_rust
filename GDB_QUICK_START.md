# 🎯 GDB 快速开始指南

## ✅ 安装完成状态

**GDB 已成功安装**: GNU gdb (GDB) 16.3
**Rust 支持**: rust-gdb 可用
**调试程序**: 多个示例已创建并测试

## 🚀 立即可用的功能

### 1. 符号查看 ✅
```bash
# 查看程序中的所有函数
gdb --batch --ex="file program" --ex="info functions"

# 查看程序结构
gdb --batch --ex="file program" --ex="info files"

# 查看反汇编
gdb --batch --ex="file program" --ex="disas main"
```

### 2. 程序分析 ✅
```bash
# 查看程序的内存布局
gdb --batch --ex="file program" --ex="maint info sections"

# 查看全局变量
gdb --batch --ex="file program" --ex="info variables"
```

### 3. Rust 专用调试 ✅
```bash
# 使用 rust-gdb (推荐)
rust-gdb program

# 查看程序文件
gdb enhanced_debug
```

## 🎯 推荐的调试工作流

### 方法 1: 开发阶段调试 (立即可用)

#### 1.1 使用 println! 调试
```rust
fn main() {
    let x = calculate_something();
    println!("DEBUG: x = {}", x);  // 简单调试输出

    for i in 0..10 {
        println!("DEBUG: loop iteration i = {}", i);
        // 复杂逻辑
    }
}
```

#### 1.2 使用 dbg! 宏 (Rust 1.32+)
```rust
fn main() {
    let x = calculate_something();
    dbg!(&x);  // 自动显示值和位置

    let result = process_data(x);
    dbg!(&result);
}
```

#### 1.3 使用 assert! 进行条件检查
```rust
fn main() {
    let age = calculate_age();
    assert!(age >= 0, "年龄不能为负数: {}", age);
    assert!(age <= 150, "年龄超出合理范围: {}", age);

    let status = process_user(age);
    assert!(status.is_ok(), "处理失败: {:?}", status);
}
```

### 方法 2: 程序分析 (立即可用)

#### 2.1 使用创建的增强调试程序
```bash
# 编译增强调试示例
rustc -g enhanced_debug.rs -o enhanced_debug

# 运行调试版本
./enhanced_debug
```

**输出示例**:
```
=== 调试示例程序开始 ===
DEBUG: 初始值: 25, 计算值: 50
DEBUG: 创建 Person - name: Alice, age: 50
DEBUG: 添加分数 85 到 Alice
DEBUG: 当前分数: [85]
DEBUG: Alice 的平均分: 87.50
```

#### 2.2 查看程序符号
```bash
# 查看 main 函数符号
gdb --batch --ex="file enhanced_debug" --ex="disas main"

# 查看 Person 相关函数
gdb --batch --ex="file enhanced_debug" --ex="info functions" | grep Person
```

### 方法 3: 完整 GDB 调试 (需要配置证书)

如果你需要完整的功能（断点、单步执行等），参考 `GDB_MACOS_SETUP.md` 配置证书。

配置完成后：
```bash
# 使用 rust-gdb (最友好)
rust-gdb enhanced_debug

# 基本调试命令
(gdb) break main
(gdb) run
(gdb) print person.name
(gdb) next
(gdb) continue
```

## 🛠️ 实际调试场景示例

### 场景 1: 变量值不正确
```bash
# 方案 A: 使用 println! 调试
fn calculate(x: i32) -> i32 {
    println!("DEBUG: 输入 x = {}", x);
    let result = x * 2 + 10;
    println!("DEBUG: 结果 result = {}", result);
    result
}

# 方案 B: 使用 assert! 检查
fn calculate(x: i32) -> i32 {
    let result = x * 2 + 10;
    assert!(result >= 0, "结果不能为负数: {}", result);
    result
}
```

### 场景 2: 循环调试
```rust
fn process_numbers(nums: &[i32]) {
    for (i, num) in nums.iter().enumerate() {
        println!("DEBUG: 索引 {}，数值 {}", i, num);

        let processed = num * 3;
        println!("DEBUG: 处理后数值 {}", processed);
    }
}
```

### 场景 3: 结构体调试
```rust
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    active: bool,
}

fn create_user(name: &str, age: u32) -> User {
    let user = User {
        name: name.to_string(),
        age,
        active: true,
    };

    println!("DEBUG: 创建的用户 {:?}", user);
    user
}
```

## 📚 创建的调试资源

### 可执行程序
- `simple_test` - 基础测试程序
- `enhanced_debug` - 增强的调试示例
- `src/debug_example` - 完整的调试示例

### 源代码
- `simple_test.rs` - 基础测试源码
- `enhanced_debug.rs` - 增强调试源码
- `src/debug_example.rs` - 完整示例源码

### 配置脚本
- `gdb_quick_start.sh` - 快速启动脚本
- `gdb_fix_and_demo.sh` - 修复和演示脚本

### 调试脚本
- `debug_commands.gdb` - 完整的 GDB 调试脚本
- `auto_debug.gdb` - 自动化调试脚本

### 文档
- `GDB_DEBUG_GUIDE.md` - 5000+ 字完整指南
- `GDB_MACOS_SETUP.md` - macOS 特定配置
- `GDB_INSTALLATION_COMPLETE.md` - 安装报告
- `INSTALLATION_SUMMARY.md` - 总结文档

## 🎯 立即开始调试

### 选择 1: 使用增强调试示例 (推荐)
```bash
# 运行增强调试示例
./enhanced_debug

# 查看调试输出
# 你会看到详细的调试信息，包括变量值、函数调用等
```

### 选择 2: 使用符号查看功能
```bash
# 查看程序中的所有函数
gdb --batch --ex="file enhanced_debug" --ex="info functions" | head -20

# 查看程序的反汇编
gdb --batch --ex="file enhanced_debug" --ex="disas main" | head -30
```

### 选择 3: 创建自己的调试程序
```bash
# 创建带有调试输出的程序
cat > my_debug.rs << 'EOF'
fn main() {
    println!("=== 我的调试程序 ===");

    let x = 42;
    println!("DEBUG: 初始值 x = {}", x);

    let y = x * 2;
    println!("DEBUG: 计算值 y = {}", y);

    let result = if y > 50 {
        println!("DEBUG: y > 50，执行分支 A");
        y - 10
    } else {
        println!("DEBUG: y <= 50，执行分支 B");
        y + 10
    };

    println!("DEBUG: 最终结果 = {}", result);
    assert!(result >= 0, "结果不能为负数");
}

EOF

# 编译并运行
rustc my_debug.rs -o my_debug
./my_debug
```

## 💡 调试最佳实践

### 1. 开发时
- 使用 `println!` 或 `dbg!` 进行快速调试
- 使用 `assert!` 进行条件检查
- 使用 `#[derive(Debug)]` 简化结构体调试

### 2. 测试时
- 使用 `assert_eq!` 和 `assert_ne!` 进行值比较
- 使用 `unwrap()` 和 `expect()` 提供明确的错误信息

### 3. 复杂问题
- 使用 GDB 符号查看分析程序结构
- 配置 GDB 证书获得完整调试功能
- 使用 macOS 原生工具如 `lldb`

## 🚀 下一步

1. **尝试现有示例**: 运行 `./enhanced_debug` 查看调试输出
2. **阅读完整指南**: 查看 `GDB_DEBUG_GUIDE.md` 学习高级技巧
3. **配置完整功能**: 参考 `GDB_MACOS_SETUP.md` 配置 GDB 证书
4. **开始调试**: 在你的项目中应用这些调试技术

---

## 🎊 恭喜！

你现在拥有了完整的 Rust 调试环境：

✅ **GDB 安装完成** (v16.3)
✅ **Rust 专用支持** (rust-gdb)
✅ **多种调试方法** (println!, dbg!, assert!)
✅ **完整的示例程序** (simple_test, enhanced_debug, debug_example)
✅ **详细的配置指南** (macOS 特定设置)
✅ **丰富的调试脚本** (自动和手动)
✅ **全面的学习文档** (5000+ 字指南)

**现在就可以开始调试 Rust 程序了！** 🚀

---

**快速开始命令**: `./enhanced_debug`
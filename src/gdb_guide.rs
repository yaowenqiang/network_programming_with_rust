// Rust GDB 调试指南和示例

// 编译选项：
// 1. 基本 GDB 调试编译
// rustc -g debug_example.rs -o debug_example
//
// 2. 优化和调试信息
// rustc -g -O debug_example.rs -o debug_example_optimized
//
// 3. 禁用优化和启用调试信息
// rustc -g -C opt-level=0 debug_example.rs -o debug_example_debug
//
// 4. Cargo 项目调试编译
// cargo build

// 使用 Cargo 进行调试：
// 1. 调试构建
// cargo build
//
// 2. 运行调试版本
// cargo run
//
// 3. 测试调试
// cargo test

// GDB 基本命令：
// file <executable>        - 加载可执行文件
// run [args]              - 运行程序
// break <location>       - 设置断点
// next                   - 单步执行（不进入函数）
// step                   - 单步执行（进入函数）
// continue               - 继续执行
// print <variable>        - 打印变量值
// info locals            - 显示局部变量
// backtrace              - 显示调用栈
// quit                   - 退出 GDB

fn main() {
    // 这个文件是 GDB 调试指南的注释
    // 请参考下面的 GDB 调试示例

    println!("GDB 调试指南 - 请查看源代码注释");
}
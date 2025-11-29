#!/bin/bash

echo "=== macOS GDB 权限修复和演示 ===\n"

echo "🔍 问题诊断:"
echo "你遇到的错误是 macOS 上的 GDB 权限问题。"
echo "GDB 已安装但需要特殊权限才能调试程序。"
echo ""

echo "📋 当前状态检查:"
echo "GDB 版本: $(gdb --version | head -1)"
echo "测试程序: $(ls -la simple_test 2>/dev/null | awk '{print $9, $5}')"
echo ""

echo "🚨 macOS GDB 权限问题:"
echo "错误 'Don't know how to run' 表示 GDB 无法控制程序执行。"
echo "这是 macOS System Integrity Protection (SIP) 的安全限制。"
echo ""

echo "🔧 解决方案选项:"
echo ""

echo "选项 1: 使用替代调试方法 (推荐 - 立即可用)"
echo ""
echo "1.1 使用 println! 调试法:"
cat << 'EOF'
// 在你的 Rust 代码中添加调试输出
fn main() {
    let x = 42;
    println!("DEBUG: x = {}", x);  // 简单调试

    for i in 0..3 {
        println!("DEBUG: loop i = {}", i);  // 循环调试
    }
}
EOF

echo ""
echo "1.2 使用 assert! 和 panic! 调试:"
cat << 'EOF'
fn main() {
    let value = 42;

    // 检查条件，如果 false 则 panic 并显示变量值
    assert!(value == 42, "value 应该是 42，实际是 {}", value);

    // 使用 debug 宏
    dbg!(&value);  // Rust 1.32+ 的调试宏
}
EOF

echo ""
echo "选项 2: 修复 GDB 权限 (完整功能)"
echo ""
echo "2.1 创建调试证书 (需要手动操作):"
echo "步骤:"
echo "  1. 打开 '钥匙串访问' (Keychain Access)"
echo "  2. 菜单 → 钥匙串访问 → 证书助理 → 创建证书"
echo "  3. 名称: gdb-cert"
echo "  4. 身份类型: 自签名根证书"
echo "  5. 证书类型: 代码签名"
echo "  6. 勾选: 让我覆盖默认值"
echo "  7. 点击 '继续'"
echo "  8. 有效期: 3650 天"
echo "  9. 输入你的邮箱和组织信息"
echo "10. 点击 '继续' 直到完成"
echo ""
echo "2.2 设置证书信任:"
echo "步骤:"
echo "  1. 在钥匙串中找到 'gdb-cert'"
echo "  2. 双击证书打开详情"
echo "  3. 展开 '信任' 部分"
echo "  4. '代码签名' 设置为 '始终信任'"
echo "  5. 关闭窗口并保存更改"
echo ""

echo "2.3 对 GDB 进行代码签名:"
echo "执行以下命令:"
echo "  codesign -fs gdb-cert $(which gdb)"
echo ""

echo "现在测试一下是否可以找到证书:"
if security find-identity -v -p codesigning | grep -q gdb-cert; then
    echo "✅ 找到 gdb-cert 证书"
    echo "执行代码签名:"
    codesign -fs gdb-cert $(which gdb)

    if [ $? -eq 0 ]; then
        echo "✅ GDB 代码签名成功!"
        echo ""
        echo "🎯 测试修复后的 GDB:"
        echo "现在可以执行: rust-gdb simple_test"
        echo "或者: gdb simple_test"
    else
        echo "❌ 代码签名失败，可能需要手动操作"
    fi
else
    echo "❌ 未找到 gdb-cert 证书"
    echo "需要先按照上述步骤创建证书"
fi

echo ""
echo "选项 3: 使用现有的符号查看功能"
echo ""
echo "这些功能无需特殊权限:"

echo "3.1 查看程序符号:"
gdb --batch --ex="file simple_test" --ex="info functions" --ex="quit" 2>&1 | head -10

echo ""
echo "3.2 查看程序信息:"
gdb --batch --ex="file simple_test" --ex="info files" --ex="quit" 2>&1 | head -15

echo ""
echo "3.3 查看反汇编:"
gdb --batch --ex="file simple_test" --ex="disas main" --ex="quit" 2>&1 | head -20

echo ""
echo "选项 4: 使用其他调试工具"
echo ""
echo "4.1 使用 Rust 内置调试:"
echo "  - RUST_LOG=debug cargo run  # 启用调试日志"
echo "  - cargo build --verbose    # 详细编译信息"
echo "  - rustc --emit asm        # 查看生成的汇编"
echo ""

echo "4.2 使用系统工具:"
echo "  - lldb program_name       # macOS 原生调试器"
echo "  - dtruss program_name     # 系统调用跟踪"
echo "  - instruments              # 性能分析"
echo ""

echo "🎯 推荐的调试策略:"
echo ""
echo "1. 开发阶段: 使用 println! 和 dbg! 宏"
echo "2. 简单问题: 使用 assert! 和 panic!"
echo "3. 复杂问题: 配置 GDB 证书获得完整功能"
echo "4. 性能问题: 使用 lldb 或 instruments"
echo ""

echo "📚 立即可用的调试示例:"
echo ""

# 创建一个增强的调试示例
cat > enhanced_debug.rs << 'EOF'
use std::collections::HashMap;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    scores: Vec<i32>,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        println!("DEBUG: 创建 Person - name: {}, age: {}", name, age);
        Person {
            name: name.to_string(),
            age,
            scores: Vec::new(),
        }
    }

    fn add_score(&mut self, score: i32) {
        println!("DEBUG: 添加分数 {} 到 {}", score, self.name);
        self.scores.push(score);
        println!("DEBUG: 当前分数: {:?}", self.scores);
    }

    fn average_score(&self) -> Option<f64> {
        println!("DEBUG: 计算平均分，分数列表: {:?}", self.scores);

        if self.scores.is_empty() {
            println!("DEBUG: 没有分数，返回 None");
            None
        } else {
            let sum: i32 = self.scores.iter().sum();
            let avg = sum as f64 / self.scores.len() as f64;
            println!("DEBUG: 总分: {}, 数量: {}, 平均: {:.2}", sum, self.scores.len(), avg);
            Some(avg)
        }
    }
}

fn main() {
    println!("=== 调试示例程序开始 ===");

    // 使用 dbg! 宏 (Rust 1.32+)
    let initial_value = 25;
    let calculated_value = dbg!(initial_value * 2);

    println!("DEBUG: 初始值: {}, 计算值: {}", initial_value, calculated_value);

    let mut person = Person::new("Alice", calculated_value as u32);
    println!("DEBUG: 创建的 Person: {:?}", person);

    // 添加一些分数
    for score in &[85, 92, 78, 95] {
        person.add_score(*score);
    }

    // 计算平均分
    match person.average_score() {
        Some(avg) => println!("DEBUG: {} 的平均分: {:.2}", person.name, avg),
        None => println!("DEBUG: {} 没有分数", person.name),
    }

    // 使用 assert! 进行条件检查
    assert!(person.age >= 18, "年龄必须 >= 18，实际是 {}", person.age);
    assert!(!person.scores.is_empty(), "分数列表不能为空");

    // 最终状态检查
    println!("DEBUG: 最终 Person 状态: {:?}", person);

    println!("=== 调试示例程序结束 ===");
}

// 辅助函数用于测试
fn debug_vector_operations() {
    println!("=== 向量操作调试 ===");

    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("DEBUG: 初始向量: {:?}", numbers);

    // 逐个处理元素并显示调试信息
    for (index, num) in numbers.iter_mut().enumerate() {
        println!("DEBUG: 处理索引 {}, 当前值: {}", index, num);
        *num *= 2;
        println!("DEBUG: 翻倍后索引 {}, 新值: {}", index, num);
    }

    println!("DEBUG: 最终向量: {:?}", numbers);
    println!("DEBUG: 向量长度: {}", numbers.len());
    println!("DEBUG: 向量容量: {}", numbers.capacity());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_debug() {
        let mut person = Person::new("Test", 20);
        person.add_score(100);

        let avg = person.average_score();
        assert!(avg.is_some(), "有分数时平均分不应为 None");
        assert_eq!(avg.unwrap(), 100.0, "平均分应为 100.0");
    }
}
EOF

echo "编译增强调试示例:"
rustc -g enhanced_debug.rs -o enhanced_debug

echo ""
echo "运行增强调试示例:"
./enhanced_debug

echo ""
echo "🎯 现在你拥有了多种调试选项:"
echo ""
echo "✅ 立即可用:"
echo "  - println! 宏调试"
echo "  - dbg! 宏 (Rust 1.32+)"
echo "  - assert! 条件检查"
echo "  - 程序符号查看"
echo ""
echo "🔧 需要配置:"
echo "  - GDB 证书 (获得完整调试功能)"
echo "  - lldb (macOS 原生调试器)"
echo ""
echo "📚 参考资源:"
echo "  - GDB_DEBUG_GUIDE.md (完整指南)"
echo "  - GDB_MACOS_SETUP.md (权限配置)"
echo "  - Rust 调试文档"
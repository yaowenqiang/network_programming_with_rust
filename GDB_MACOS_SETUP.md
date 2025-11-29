# macOS GDB 设置指南

## 🎯 状态更新

✅ **GDB 已成功安装**: GNU gdb (GDB) 16.3

## 🔧 macOS GDB 配置步骤

在 macOS 上，GDB 需要特殊权限才能调试程序。你需要创建并安装一个证书。

### 方法 1: 自动化设置脚本

```bash
# 运行自动化设置脚本
curl -s https://gist.githubusercontent.com/gdbinit/8f2d18bf25aa8b767947/raw/70271a5d14bd45a46141ab9a27483891b3566846/gdb-cert.sh | bash
```

### 方法 2: 手动创建证书

#### 步骤 1: 打开钥匙串访问
1. 打开 "钥匙串访问" (Keychain Access) 应用
2. 菜单 → 钥匙串访问 → 证书助理 → 创建证书

#### 步骤 2: 配置证书信息
- **名称**: `gdb-cert` (任意名称)
- **身份类型**: 自签名根证书
- **证书类型**: 代码签名
- **勾选**: 让我覆盖默认值

#### 步骤 3: 证书设置
- **有效期**: 3650 天 (10年)
- **邮件地址**: 你的邮箱
- **组织名称**: 你的名字或组织
- **组织单位**: (可选)

#### 步骤 4: 扩展密钥使用
- **不勾选** 所有默认选项
- **勾选**: 代码签名

#### 步骤 5: 设置信任
- 找到创建的证书 → 双击
- **信任**: 设为"始终信任"
- **代码签名**: 设为"始终信任"

### 步骤 3: 代码签名 GDB

```bash
# 重新签名 GDB (需要输入密码)
codesign -fs gdb-cert $(which gdb)

# 验证签名
codesign -v $(which gdb)
```

### 步骤 4: 测试 GDB

```bash
# 创建测试文件
echo 'fn main() { println!("Hello GDB!"); }' > test_gdb.rs

# 编译测试程序
rustc -g test_gdb.rs -o test_gdb

# 使用 GDB 调试
gdb test_gdb

# 在 GDB 中测试
(gdb) break main
(gdb) run
(gdb) print "调试正常工作！"
(gdb) continue
(gdb) quit
```

## 🚀 快速开始调试你的 Rust 程序

现在你可以开始调试之前创建的 Rust 程序：

```bash
# 使用调试脚本
gdb -x debug_commands.gdb src/debug_example

# 或者手动调试
gdb src/debug_example
```

## 💡 常见问题解决

### 问题 1: "Permission denied" 或 "Unable to find Mach task"
**解决**: 确保代码签名正确执行：
```bash
codesign -fs gdb-cert $(which gdb)
```

### 问题 2: 证书无法使用
**解决**: 在钥匙串访问中：
1. 找到证书 → 右键 → 显示简介
2. 信任 → 代码签名 → 设为"始终信任"

### 问题 3: 仍然提示权限问题
**解决**: 重启终端或执行：
```bash
killall gdb
gdb your_program
```

## 🎯 成功标志

如果配置正确，你应该能看到：
- GDB 可以正常启动
- 可以设置断点
- 可以运行和调试程序
- 没有"permission denied"错误

现在你可以愉快地使用 GDB 调试 Rust 程序了！🎉
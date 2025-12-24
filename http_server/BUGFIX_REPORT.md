# Bug 修复报告

## 概述

本文档记录了对 HTTP 服务器代码中发现的 bug 的分析与修复过程。

## 初始发现的 Bug

### 1. 严重 Bug：`read_exact` 阻塞问题

**位置**: `src/server.rs:40`

**问题描述**:
```rust
match stream.read_exact(&mut buffer) {
```

`read_exact()` 会阻塞直到读取满 1024 字节。如果 HTTP 请求小于 1024 字节（大多数请求都小于这个大小），服务器会一直等待，直到：
- 客户端发送足够多的数据填满缓冲区
- 客户端关闭连接
- 发生超时

**影响**: 服务器会挂起无法响应正常的小请求。

**修复方案**:
```rust
match stream.read(&mut buffer) {
    Ok(bytes_read) => {
        let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);
        let response = match Request::try_from(&buffer[..bytes_read]) {
```

使用 `read()` 方法并处理实际读取的字节数，只解析实际接收到的数据。

---

### 2. 拼写错误

**位置**: `src/server.rs:68`

**问题描述**:
```rust
Err(e) => println!("Failed to read from connecion: {e}"),
```
`connecion` 拼写错误，应为 `connection`。

**修复方案**:
```rust
Err(e) => println!("Failed to read from connection: {e}"),
```

---

### 3. HTTP 状态码短语错误

**位置**: `src/http/status_code.rs:15`

**问题描述**:
```rust
Self::BadRequest =>"Bad BadRequest",
```
HTTP 400 的标准短语是 "Bad Request"，而不是 "Bad BadRequest"。

**修复方案**:
```rust
Self::BadRequest =>"Bad Request",
```

---

### 4. 无意义的代码

**位置**: `src/server.rs:65`

**问题描述**:
```rust
if let Err(e) = response.send(&mut stream) {
    println!("Failed to send response: {}", e);
    Response::new(StatusCode::BadRequest, None);  // 创建了但立即丢弃
}
```
创建了 `Response` 但没有使用它，这行代码没有任何效果。

**修复方案**:
```rust
if let Err(e) = response.send(&mut stream) {
    println!("Failed to send response: {}", e);
}
```

---

### 5. 路径处理问题

**位置**: `src/website_handler.rs:28`

**问题描述**:
```rust
path => match self.read_file(path) {
```
`path` 包含前导斜杠（如 `/style.css`），在 `read_file` 中会生成 `public_path//style.css`（双斜杠）。

**修复方案**:
```rust
path => match self.read_file(&path[1..]) {
```
使用 `&path[1..]` 去掉前导斜杠。

---

## 运行时发现的 Bug

### 6. `get_next_word` 函数解析错误

**位置**: `src/http/request.rs:109-129`

**问题现象**:
```
Failed to parse request: Invalid Protocol
```

**问题分析**:

原始实现存在以下问题：

1. **没有处理前导空格**
   ```rust
   // 原始代码
   for (i,c) in request.chars().enumerate() {
       if c == ' ' || c == '\n' {
           return Some((&request[..i], &request[i+1..]));
       }
   }
   ```
   当输入是 `" / HTTP/1.1\r\n"` 时，会返回 `("", "/ HTTP/1.1\r\n")`，导致解析出空字符串。

2. **没有处理 `\r` 字符**
   HTTP 请求使用 `\r\n` 作为行分隔符，原函数只检查空格和 `\n`，会跳过 `\r` 导致解析错误。

**解析过程示例**（原始代码）:
```
输入: "GET / HTTP/1.1\r\nHost: localhost"

第一次调用 get_next_word("GET / HTTP/1.1\r\nHost: localhost")
  返回: ("GET", " / HTTP/1.1\r\nHost: localhost")

第二次调用 get_next_word(" / HTTP/1.1\r\nHost: localhost")
  返回: ("", "/ HTTP/1.1\r\nHost: localhost")  ← 空字符串！

第三次调用 get_next_word("/ HTTP/1.1\r\nHost: localhost")
  返回: ("/", " HTTP/1.1\r\nHost: localhost")

第四次调用 get_next_word(" HTTP/1.1\r\nHost: localhost")
  返回: ("", "HTTP/1.1\r\nHost: localhost")  ← 又是空字符串！

结果: protocol 被解析为 "/HTTP/1.1" 或其他错误值
```

**修复方案**:
```rust
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    let mut iter = request.chars().enumerate();
    // Skip leading whitespace
    loop {
        match iter.next() {
            Some((_, c)) if c == ' ' || c == '\r' || c == '\n' => continue,
            Some((i, _)) => {
                // Found start of word, now find the end
                for (j, c) in iter {
                    if c == ' ' || c == '\r' || c == '\n' {
                        return Some((&request[i..j], &request[j..]));
                    }
                }
                return Some((&request[i..], ""));
            }
            None => return None,
        }
    }
}
```

**新逻辑**:
1. 先跳过所有前导空白字符（空格、`\r`、`\n`）
2. 找到单词的起始位置
3. 继续查找单词的结束位置（遇到空白字符）
4. 返回的剩余部分包含分隔符，这样下次调用时不会因为前导分隔符返回空字符串

**解析过程示例**（修复后）:
```
输入: "GET / HTTP/1.1\r\nHost: localhost"

第一次调用
  跳过前导空白: 无
  返回: ("GET", " / HTTP/1.1\r\nHost: localhost")

第二次调用
  跳过前导空白: " " (一个空格)
  返回: ("/", " HTTP/1.1\r\nHost: localhost")

第三次调用
  跳过前导空白: " " (一个空格)
  返回: ("HTTP/1.1", "\r\nHost: localhost")

结果: 正确解析出 method=GET, path=/, protocol=HTTP/1.1
```

---

## 修复文件清单

| 文件 | 修改内容 |
|------|----------|
| `src/server.rs` | 修复 `read_exact` 阻塞问题 |
| `src/server.rs` | 修复拼写错误 `connecion` → `connection` |
| `src/server.rs` | 删除无意义的 `Response` 创建代码 |
| `src/http/status_code.rs` | 修复状态码短语 `Bad BadRequest` → `Bad Request` |
| `src/website_handler.rs` | 修复路径处理 `path` → `&path[1..]` |
| `src/http/request.rs` | 重写 `get_next_word` 函数以正确处理空白字符 |

---

## 验证

所有修复完成后，代码通过编译检查：
```bash
$ cargo check
    Checking http_server v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
```

---

## 总结

本次修复共解决 6 个 bug：

1. **1 个严重 bug** (`read_exact` 阻塞) - 会导致服务器无法正常工作
2. **1 个运行时 bug** (`get_next_word` 解析错误) - 会导致所有请求解析失败
3. **4 个次要 bug** (拼写错误、状态码短语、无意义代码、路径处理)

最关键的修复是第 1 和第 6 个 bug，它们直接影响服务器的核心功能。其他修复提高了代码质量和 HTTP 标准合规性。

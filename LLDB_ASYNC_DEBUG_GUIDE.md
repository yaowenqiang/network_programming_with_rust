# 🔧 LLDB 异步 Rust 程序调试完整指南

## 📋 目录

1. [异步程序的特殊挑战](#异步调试挑战)
2. [LLDB 异步调试基础](#lldb-异步基础)
3. [关键调试位置](#关键调试位置)
4. [LLDB 异步调试命令](#lldb-异步命令)
5. [实用调试技巧](#实用调试技巧)
6. [多线程调试](#多线程调试)
7. [状态机和状态调试](#状态机调试)
8. [Future 和 Task 调试](#future-task-调试)
9. [Tokio 生态系统调试](#tokio-调试)
10. [常见问题和解决方案](#常见问题解决)
11. [最佳实践](#最佳实践)

---

## 异步调试挑战

### 🔍 异步程序的特殊性

异步 Rust 程序调试相比同步程序有以下挑战：

#### 1. **执行模型差异**
```rust
// 同步代码
let result = some_function();  // 在这里可以断点

// 异步代码
let result = some_async_function().await;  // 断点可能停在错误位置
```

#### 2. **线程复杂性**
- **运行时线程调度**: 异步代码在不同线程间切换
- **栈帧丢失**: `await` 会导致栈帧在不同线程中
- **线程间通信**: 数据在线程间传递需要特殊处理

#### 3. **内存复杂性**
- **Pin 和 Unpin**: 异步代码使用这些概念
- **Future 状态**: 需要跟踪 Future 的状态变化
- **引用和生命周期**: 异步中的借用规则更复杂

#### 4. **运行时行为**
- **非顺序执行**: 异步代码不按代码顺序执行
- **轮询机制**: Future 的轮询过程难以预测
- **取消和超时**: 异步操作的取消机制复杂

---

## LLDB 异步调试基础

### 🎯 基本概念

#### 1. **异步函数断点**
```bash
# 在 async 函数入口设置断点
(lldb) breakpoint set --name my_async_function

# 在 Future 实现（Poll 方法）中设置断点
(lldb) breakpoint set --name "my_async_future::poll"
```

#### 2. **异步栈查看**
```bash
# 查看调用栈（可能显示异步相关的帧）
(lldb) thread backtrace

# 查看特定线程的栈
(lldb) thread backtrace --all
```

#### 3. **异步状态检查**
```bash
# 查看 Future 的状态
(lldb) frame variable future_state

# 查看异步任务的内部状态
(lldb) frame variable task_waker
```

---

## 关键调试位置

### 🎯 1. 异步函数入口

```rust
async fn process_data(data: Vec<i32>) -> Result<i32, String> {
    println!("DEBUG: 开始处理数据: {:?}", data);

    // 👈 在这里设置断点来跟踪异步函数开始
    let result = data.iter().sum();

    println!("DEBUG: 处理结果: {}", result);
    Ok(result)
}
```

**LLDB 调试**:
```bash
(lldb) breakpoint set --name process_data
(lldb) run
# 程序会在函数入口处暂停
```

### 🎯 2. .await 调用点

```rust
async fn complex_operation() -> i32 {
    let data = vec![1, 2, 3];

    // 👈 在 .await 前设置断点
    let result1 = async_step1(data.clone()).await;

    // 👈 在 .await 后设置断点查看状态变化
    let result2 = async_step2(result1).await;

    result1 + result2
}
```

**LLDB 调试**:
```bash
(lldb) breakpoint set --name complex_operation
(lldb) breakpoint set --line complex_operation.rs:15  # 第一个 .await
(lldb) breakpoint set --line complex_operation.rs:17  # 第二个 .await
```

### 🎯 3. async 块入口

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 👈 在 main async 函数入口设置断点
    let handles: Vec<_> = vec![]

    for i in 0..3 {
        let handle = tokio::spawn(async move {
            process_item(i).await
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
```

**LLDB 调试**:
```bash
(lldb) breakpoint set --name "main::{{closure}}"  # async main 块
(lldb) run
```

---

## LLDB 异步调试命令

### 🔧 1. 线程管理

```bash
# 查看所有运行的线程
(lldb) thread list

# 切换到特定线程
(lldb) thread select <thread_id>

# 查看线程的寄存器和栈
(lldb) thread select 2
(lldb) thread backtrace
```

### 🔧 2. 异步变量查看

```bash
# 查看异步任务的变量
(lldb) frame variable async_variable

# 查看 Future 的状态
(lldb) expression future.is_ready()

# 查看 Waker 的状态
(lldb) frame variable waker
```

### 🔧 3. 内存和 Pin 调试

```bash
# 查看被 Pin 的数据
(lldb) frame variable pinned_data

# 查看数据的内存地址
(lldb) expression &pinned_data

# 验证 Pin 保证
(lldb) expression pinned_data.as_ptr()
```

### 🔧 4. 异步流调试

```bash
# 查看流的状态
(lldb) frame variable stream

# 查看流迭代器
(lldb) frame variable iterator

# 查看流缓冲区
(lldb) memory read --size 256 --format hex &stream
```

---

## 实用调试技巧

### 🎯 1. 异步函数调试模式

#### 模式 A: 入口和出口标记
```rust
async fn debuggable_async_function(input: i32) -> Result<i32, String> {
    println!("DEBUG[{}]: 函数开始，输入: {}", line!(), input);

    // 异步工作
    let result = input * 2;

    // 模拟异步操作
    tokio::time::sleep(Duration::from_millis(100)).await;

    println!("DEBUG[{}]: 函数结束，输出: {}", line!(), result);

    if result > 100 {
        Err("结果太大".to_string())
    } else {
        Ok(result)
    }
}
```

#### 模式 B: 状态跟踪
```rust
#[derive(Debug)]
enum AsyncState {
    Starting,
    Processing(u32),
    Waiting,
    Completed(u32),
    Error(String),
}

async fn state_tracked_operation() -> AsyncState {
    println!("DEBUG[{}]: 操作开始", line!());

    // 初始状态
    let mut state = AsyncState::Starting;

    // 处理阶段
    for i in 1..=5 {
        state = AsyncState::Processing(i);
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    // 完成状态
    state = AsyncState::Completed(42);

    println!("DEBUG[{}]: 最终状态: {:?}", line!(), state);
    state
}
```

### 🎯 2. 异步错误调试

```rust
async fn error_prone_operation(might_fail: bool) -> Result<String, String> {
    println!("DEBUG[{}]: 开始操作，失败标志: {}", line!(), might_fail);

    tokio::time::sleep(Duration::from_millis(200)).await;

    if might_fail {
        // 👈 在错误发生前设置断点
        let error_msg = format!("操作失败: {}", line!());
        println!("DEBUG[{}]: 发生错误: {}", line!(), error_msg);
        Err(error_msg)
    } else {
        let success_msg = format!("操作成功: {}", line!());
        println!("DEBUG[{}]: 操作成功: {}", line!(), success_msg);
        Ok(success_msg)
    }
}
```

### 🎯 3. 异步取消调试

```rust
use tokio::sync::CancellationToken;

async fn cancellable_operation(token: CancellationToken) -> Result<String, String> {
    println!("DEBUG[{}]: 开始可取消操作", line!());

    let mut counter = 0;

    loop {
        // 检查是否收到取消信号
        if token.is_cancelled() {
            println!("DEBUG[{}]: 操作被取消", line!());
            return Err("操作被取消".to_string());
        }

        // 模拟工作
        counter += 1;
        println!("DEBUG[{}]: 工作步骤: {}", line!(), counter);

        tokio::time::sleep(Duration::from_millis(100)).await;

        if counter >= 5 {
            break;
        }
    }

    let result = format!("完成 {} 个步骤", counter);
    println!("DEBUG[{}]: 操作完成: {}", line!(), result);
    Ok(result)
}
```

---

## 多线程调试

### 🔧 1. 线程间通信调试

```rust
use tokio::sync::{mpsc, oneshot};

async fn debug_channels() {
    let (tx, mut rx) = mpsc::channel::<String>(10);

    // 启动生产者任务
    tokio::spawn(async move {
        for i in 0..=3 {
            let msg = format!("消息 {}", i);
            println!("DEBUG[Producer]: 发送: {}", msg);
            tx.send(msg).await.unwrap();
        }
    });

    // 消费者任务
    while let Some(msg) = rx.recv().await {
        println!("DEBUG[Consumer]: 收到: {}", msg);
        // 👈 在这里设置断点查看消费者状态
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
```

**LLDB 调试**:
```bash
# 在不同线程设置断点
(lldb) thread list
(lldb) thread select 1  # 选择生产者线程
(lldb) breakpoint set --line channels.rs:20
(lldb) continue
```

### 🔧 2. 共享状态调试

```rust
use tokio::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug)]
struct SharedState {
    counter: AtomicU64,
    data: Mutex<Vec<String>>,
    config: RwLock<HashMap<String, String>>,
}

impl SharedState {
    fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
            data: Mutex::new(Vec::new()),
            config: RwLock::new(HashMap::new()),
        }
    }

    fn increment_counter(&self, line: u32) -> u64 {
        let old = self.counter.fetch_add(1, Ordering::Relaxed);
        println!("DEBUG[{}]: 计数器: {} -> {}", line!(), old - 1, old);
        old
    }
}

async fn debug_shared_state() {
    let state = Arc::new(SharedState::new());
    let mut handles = Vec::new();

    for i in 0..=3 {
        let state_clone = Arc::clone(&state);

        let handle = tokio::spawn(async move {
            // 修改共享状态
            let current_count = state_clone.increment_counter(i);

            // 写入数据
            {
                let mut data = state_clone.data.lock().await;
                data.push(format!("任务 {} 的数据", i));
                println!("DEBUG[Task{}]: 数据长度: {}", i, data.len());
            }
        });

        handles.push(handle);
    }

    // 等待所有任务完成
    for handle in handles {
        handle.await;
    }

    let final_count = state.counter.load(Ordering::Relaxed);
    println!("DEBUG[Main]: 最终计数值: {}", final_count);
}
```

---

## 状态机和状态调试

### 🎯 1. 异步状态机

```rust
#[derive(Debug, Clone)]
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Failed(String),
}

#[derive(Debug)]
struct ConnectionEvent {
    timestamp: std::time::Instant,
    state: ConnectionState,
    data: Option<String>,
}

struct AsyncStateMachine {
    state: ConnectionState,
    events: Vec<ConnectionEvent>,
}

impl AsyncStateMachine {
    fn new() -> Self {
        Self {
            state: ConnectionState::Disconnected,
            events: Vec::new(),
        }
    }

    async fn connect(&mut self) -> Result<(), String> {
        println!("DEBUG[StateMachine]: 开始连接，当前状态: {:?}", self.state);

        // 状态转换
        self.state = ConnectionState::Connecting;
        self.events.push(ConnectionEvent {
            timestamp: std::time::Instant::now(),
            state: self.state.clone(),
            data: None,
        });

        // 模拟连接过程
        tokio::time::sleep(Duration::from_millis(500)).await;

        // 可能的失败
        if rand::random::<f32>() < 0.3 {
            self.state = ConnectionState::Failed("连接超时".to_string());
            self.events.push(ConnectionEvent {
                timestamp: std::time::Instant::now(),
                state: self.state.clone(),
                data: Some("连接失败".to_string()),
            });

            println!("DEBUG[StateMachine]: 连接失败");
            return Err("连接失败".to_string());
        }

        // 成功连接
        self.state = ConnectionState::Connected;
        self.events.push(ConnectionEvent {
            timestamp: std::time::Instant::now(),
            state: self.state.clone(),
            data: Some("连接成功".to_string()),
        });

        println!("DEBUG[StateMachine]: 连接成功");
        Ok(())
    }
}
```

**LLDB 调试**:
```bash
# 在状态转换处设置断点
(lldb) breakpoint set --file state_machine.rs --line 25  Connecting 状态
(lldb) breakpoint set --file state_machine.rs --line 35  Failed 状态
(lldb) frame variable self.state
(lldb) frame variable self.events
```

### 🎯 2. 复杂异步流程调试

```rust
#[derive(Debug)]
struct WorkflowState {
    current_step: u32,
    total_steps: u32,
    data: Vec<String>,
    errors: Vec<String>,
}

impl WorkflowState {
    fn new() -> Self {
        Self {
            current_step: 0,
            total_steps: 5,
            data: Vec::new(),
            errors: Vec::new(),
        }
    }

    async fn execute_workflow(&mut self) -> Result<(), String> {
        println!("DEBUG[Workflow]: 开始工作流，总步骤: {}", self.total_steps);

        for step in 1..=self.total_steps {
            println!("DEBUG[Workflow]: 开始步骤 {}", step);

            // 更新当前步骤
            self.current_step = step;

            // 模拟步骤执行
            tokio::time::sleep(Duration::from_millis(300)).await;

            // 模拟可能的错误
            if step == 3 && rand::random::<f32>() < 0.5 {
                let error_msg = format!("步骤 {} 失败", step);
                self.errors.push(error_msg.clone());
                println!("DEBUG[Workflow]: {}", error_msg);
                continue;
            }

            // 收集步骤数据
            let step_data = format!("步骤 {} 的数据", step);
            self.data.push(step_data);
            println!("DEBUG[Workflow]: {} 完成", step_data);
        }

        if !self.errors.is_empty() {
            Err(format!("工作流失败，错误: {:?}", self.errors))
        } else {
            println!("DEBUG[Workflow]: 工作流成功完成");
            Ok(())
        }
    }
}
```

---

## Future 和 Task 调试

### 🎯 1. 自定义 Future 调试

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

struct SlowFuture {
    started_at: std::time::Instant,
    poll_count: u64,
    delay_ms: u64,
}

impl SlowFuture {
    fn new(delay_ms: u64) -> Self {
        Self {
            started_at: std::time::Instant::now(),
            poll_count: 0,
            delay_ms,
        }
    }
}

impl Future for SlowFuture {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.poll_count += 1;

        let elapsed = self.started_at.elapsed();
        let should_complete = elapsed.as_millis() >= self.delay_ms;

        println!("DEBUG[SlowFuture]: 第 {} 次轮询，耗时: {}ms, 应完成: {}",
                 self.poll_count, elapsed.as_millis(), should_complete);

        if should_complete {
            Poll::Ready(format!("经过 {}ms 后完成", self.delay_ms))
        } else {
            Poll::Pending
        }
    }
}
```

**LLDB 调试**:
```bash
# 在轮询方法中设置断点
(lldb) breakpoint set --file slow_future.rs --line 45
(lldb) frame variable self.poll_count
(lldb) frame variable elapsed.as_millis()
```

### 🎯 2. Task 调试和跟踪

```rust
use tokio::task::JoinHandle;

#[derive(Debug)]
struct TrackedTask {
    id: u32,
    started_at: std::time::Instant,
    completed_at: Option<std::time::Instant>,
}

impl TrackedTask {
    fn new(id: u32) -> Self {
        Self {
            id,
            started_at: std::time::Instant::now(),
            completed_at: None,
        }
    }

    fn complete(&mut self) {
        self.completed_at = Some(std::time::Instant::now());
        let duration = self.completed_at.unwrap() - self.started_at;
        println!("DEBUG[Task{}]: 完成，耗时: {:?}", self.id, duration);
    }
}

async fn debug_tasks() {
    println!("DEBUG: 开始任务调试");

    let mut tasks = Vec::new();

    for i in 1..=3 {
        let task = TrackedTask::new(i);
        let task_clone = task.clone();

        let handle: JoinHandle<TrackedTask> = tokio::spawn(async move {
            println!("DEBUG[Task{}]: 开始执行", task_clone.id);

            // 模拟工作
            tokio::time::sleep(Duration::from_millis(200 + i * 100)).await;

            // 任务完成
            println!("DEBUG[Task{}]: 工作完成", task_clone.id);
            task_clone.complete();

            task_clone
        });

        tasks.push(handle);
    }

    // 等待所有任务完成
    for handle in tasks {
        let completed_task = handle.await;
        println!("DEBUG: 收到完成的任务: {:?}", completed_task);
    }
}
```

**LLDB 调试**:
```bash
# 在任务执行中设置断点
(lldb) breakpoint set --name "debug_tasks::{{closure}}"
(lldb) thread list  # 查看所有任务线程
(lldb) thread select 1  # 选择第一个任务线程
(lldb) frame variable task_clone.id
```

---

## Tokio 生态系统调试

### 🎯 1. Tokio 运行时调试

```bash
# 设置 Tokio 调试环境变量
export RUST_LOG=debug
export TOKIO_CONSOLE_LEVEL=debug

# 运行程序
RUST_LOG=debug cargo run

# 在 LLDB 中运行
RUST_LOG=debug lldb target/debug/async_debug_main
```

### 🎯 2. Tokio 工具调试

```rust
use tokio::time::{sleep, Instant};
use tokio::sync::Barrier;

async fn debug_tokio_utilities() {
    println!("DEBUG: 开始 Tokio 工具调试");

    let barrier = Arc::new(Barrier::new(3));
    let mut handles = Vec::new();

    for i in 0..3 {
        let barrier_clone = Arc::clone(&barrier);

        let handle = tokio::spawn(async move {
            println!("DEBUG[Tokio]: 任务 {} 开始", i);

            // 使用 Tokio sleep
            sleep(Duration::from_millis(100 * (i + 1))).await;

            println!("DEBUG[Tokio]: 任务 {} 到达屏障", i);
            barrier_clone.wait().await;

            println!("DEBUG[Tokio]: 任务 {} 通过屏障", i);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await;
    }

    println!("DEBUG: 所有任务完成");
}
```

### 🎯 3. Tokio 原子调试

```rust
use tokio::sync::mpsc;

async fn debug_tokio_atomics() {
    let (tx, mut rx) = mpsc::channel::<i32>(100);

    // 生产者
    tokio::spawn(async move {
        for i in 0..=10 {
            println!("DEBUG[Producer]: 发送 {}", i);
            if let Err(e) = tx.send(i).await {
                println!("DEBUG[Producer]: 发送失败: {}", e);
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    });

    // 消费者
    while let Some(value) = rx.recv().await {
        println!("DEBUG[Consumer]: 收到 {}", value);
    }
}
```

---

## 常见问题和解决方案

### 🔧 问题 1: 异步代码停在错误位置

**问题**: 断点设置在异步函数中，但程序停在 `await` 之后而不是函数开头

**解决方案**:
```bash
# 方法 1: 在异步函数的机器码中设置断点
(lldb) disassemble --name async_function_name
# 找到实际函数开始位置并设置断点

# 方法 2: 在 Future::poll 方法中设置断点
(lldb) breakpoint set --name "MyFuture::poll"

# 方法 3: 使用条件断点
(lldb) breakpoint set --name async_function --condition "input > 5"
```

### 🔧 问题 2: 无法查看异步局部变量

**问题**: `await` 后局部变量作用域混乱

**解决方案**:
```rust
async fn fix_scope_issue() -> i32 {
    let input = 42;

    // 在 .await 前保存需要调试的变量
    let debug_input = input;  // 👈 在 .await 前保存

    let result = async_operation(debug_input).await;

    println!("DEBUG: 输入: {}, 结果: {}", debug_input, result);
    result
}
```

**LLDB 调试**:
```bash
# 在 .await 前查看变量
(lldb) frame variable debug_input
(lldb) continue
```

### 🔧 问题 3. 线程切换导致的调试困难

**问题**: 异步执行在线程间切换，难以跟踪单条执行路径

**解决方案**:
```bash
# 使用线程过滤器
(lldb) settings set target.process.thread.stepping true

# 跟踪特定线程
(lldb) thread list
(lldb) thread select <thread_id>

# 使用 condition-variables
(lldb) breakpoint set --name my_async_function --condition "thread_id == 1"
```

### 🔧 问题 4. 内存泄漏和借用问题

**问题**: 异步代码中的复杂借用导致难以理解的编译错误

**解决方案**:
```rust
use std::sync::Arc;

async fn fix_borrowing_issue(data: Vec<String>) {
    // 将数据包装在 Arc 中以在异步上下文中共享
    let shared_data = Arc::new(data);

    // 在异步块中访问
    let data_clone = Arc::clone(&shared_data);
    tokio::spawn(async move {
        for item in data_clone.iter() {
            println!("DEBUG: 处理: {}", item);
        }
    }).await;
}
```

### 🔧 问题 5. 性能问题难以定位

**问题**: 异步代码的性能瓶颈难以用传统调试方法定位

**解决方案**:
```bash
# 使用性能分析工具
cargo run --bin profile  # 使用 profiling
perf record --call-graph cargo run
 Instruments --template "Time Profiler" cargo run

# 在关键位置添加性能日志
let start_time = std::time::Instant::now();
// ... 异步操作 ...
let duration = start_time.elapsed();
println!("PERF: operation took {:?}", duration);
```

---

## 最佳实践

### 🎯 1. 异步友好的代码结构

```rust
// 好的异步函数结构
#[tracing::instrument]  // 使用 tracing 进行调试
async fn well_structured_async_function(
    input: i32,
    config: &Config,
) -> Result<i32, AppError> {
    // 使用 early return 而不是深层嵌套
    if input < 0 {
        tracing::error!("输入不能为负数: {}", input);
        return Err(AppError::InvalidInput);
    }

    // 将复杂操作分解为小的异步函数
    let step1 = async_step1(input).await?;
    let step2 = async_step2(step1).await?;
    let result = combine_results(step1, step2).await?;

    tracing::info!("异步操作完成: {}", result);
    Ok(result)
}

// 使用显式错误处理
#[derive(Debug)]
enum AppError {
    NetworkError(String),
    DatabaseError(String),
    InvalidInput,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NetworkError(msg) => write!(f, "网络错误: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "数据库错误: {}", msg),
            AppError::InvalidInput => write!(f, "输入无效"),
        }
    }
}

// 将异步错误转换为同步错误
impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
```

### 🎯 2. 测试策略

```bash
# 单元测试异步函数
#[tokio::test]
async fn test_well_structured_function() {
    let result = well_structured_async_function(42, &Config::default()).await;
    assert_eq!(result, Ok(84));
}

# 集成测试
#[tokio::test]
async fn test_async_error_handling() {
    let result = well_structured_async_function(-1, &Config::default()).await;
    assert!(result.is_err());
    assert!(matches!(result, Err(AppError::InvalidInput)));
}
```

### 🎯 3. 监控和日志

```rust
use tracing::{info, warn, error, instrument};

#[instrument]
async fn monitored_async_operation(data: &[i32]) -> i32 {
    info!("开始处理 {} 个数据项", data.len());

    let mut sum = 0;
    for (i, item) in data.iter().enumerate() {
        // 为每个项目添加详细日志
        let processed_item = item * 2;
        sum += processed_item;

        info!("处理项目 {}: {} -> {}", i, item, processed_item);

        // 模拟可能的延迟
        if i % 3 == 0 {
            warn!("项目 {} 处理较慢", i);
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    info!("异步操作完成，总和: {}", sum);
    sum
}
```

---

## 快速参考

### 🚀 异步调试命令速查

| 操作 | LLDB 命令 | 说明 |
|------|------------|------|
| 查看线程 | `thread list` | 列出所有运行的线程 |
| 切换线程 | `thread select <id>` | 选择特定线程 |
| 查看异步栈 | `thread backtrace` | 查看当前线程的调用栈 |
| 查看变量 | `frame variable var` | 查看当前帧的变量 |
| 设置断点 | `breakpoint set --name func` | 在异步函数设置断点 |
| 条件断点 | `br s -n func --condition "var > value"` | 条件断点 |
| 单步执行 | `next` / `step` | 单步执行异步代码 |
| 继续执行 | `continue` | 继续执行直到下一个断点 |
| 查看内存 | `memory read --size 64 --format hex &var` | 查看异步变量内存 |

### 🔧 Rust 异步调试模式

| 模式 | 技术说明 | 适用场景 |
|------|------------|----------|
| **入口标记** | `println!` 函数开始和结束 | 快速状态跟踪 |
| **状态跟踪** | `tracing::instrument` | 结构化日志记录 |
| **错误处理** | `Result<T, E>` 模式 | 显式错误传播 |
| **测试策略** | 单元测试 + 集成测试 | 确保异步正确性 |
| **性能监控** | 时间测量 + 日志 | 性能瓶颈识别 |

### 🔧 异步调试工具链

```bash
# 1. 开发时调试
export RUST_LOG=debug
export RUST_BACKTRACE=1
cargo run

# 2. 生产环境监控
RUST_LOG=info cargo run

# 3. 性能分析
cargo run --release --features profiling
perf record --call-graph cargo run

# 4. 内存分析
valgrind --tool=massif cargo run
```

### 🔧 异步调试最佳实践总结

1. **清晰的异步结构**: 使用小而专注的异步函数
2. **显式错误处理**: 使用 `Result<T, E>` 传播错误
3. **适当的日志**: 使用 `tracing` 或 `log` 记录异步状态
4. **测试覆盖**: 单元和集成测试确保异步正确性
5. **性能意识**: 在关键异步操作中添加性能监控

### 🔧 调试工作流

1. **问题识别**: 通过日志识别异常行为
2. **本地复现**: 在调试环境中重现问题
3. **断点设置**: 在关键异步位置设置断点
4. **状态检查**: 使用 LLDB 查看异步状态和线程
5. **步进调试**: 逐步跟踪异步执行流程
6. **解决方案验证**: 修复后测试并验证

---

## 🚀 总结

异步 Rust 程序调试虽然比同步程序更复杂，但通过合适的工具、模式和实践，可以有效地调试和优化异步代码。LLDB 提供了强大的工具来处理异步程序的复杂性。

**关键成功因素**:
- 理解异步执行模型
- 掌握 LLDB 的多线程调试能力
- 使用结构化日志记录
- 编写可测试的异步代码
- 采用适当的错误处理模式

记住：异步调试需要更多的耐心和系统性的方法，但掌握后将大大提升调试效率！
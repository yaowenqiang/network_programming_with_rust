// 导入必要的库
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

// 网络编程相关库
// 文档: https://doc.rust-lang.org/std/net/index.html
use std::net::{TcpListener, TcpStream};

// 输入输出相关库
// 文档: https://doc.rust-lang.org/std/io/index.html
use std::io::{stdin, BufRead, BufReader, Error, Write};

// 标准库其他模块
// 文档: https://doc.rust-lang.org/std/index.html
use std::{env, str, thread};

// Serde序列化/反序列化库
// 文档: https://serde.rs/
// GitHub: https://github.com/serde-rs/serde
use serde_derive::{Serialize, Deserialize};

/// 三维点结构体
///
/// 用于表示三维空间中的一个点，包含x、y、z三个坐标
///
/// # 字段
/// - `x`: x坐标，无符号32位整数
/// - `y`: y坐标，无符号32位整数
/// - `z`: z坐标，无符号32位整数
///
/// # 示例
/// ```rust
/// let point = Point3D { x: 1, y: 2, z: 3 };
/// ```
#[derive(Debug, Serialize, Deserialize)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

/// 处理客户端连接
///
/// 这个函数处理来自客户端的连接请求，接收JSON格式的三维点数据，
/// 计算该点到原点的距离，并将结果返回给客户端
///
/// # 参数
/// - `stream`: TCP连接流
///
/// # 返回值
/// - `Result<(), Error>`: 如果处理成功返回Ok(())，否则返回错误
///
/// # 功能
/// 1. 接收客户端发送的JSON格式Point3D数据
/// 2. 计算三维点到原点的距离：√(x² + y² + z²)
/// 3. 将计算结果返回给客户端
///
/// # 相关文档
/// - TCP编程: https://doc.rust-lang.org/std/net/struct.TcpStream.html
/// - JSON序列化: https://serde.rs/
fn handle_client(stream: TcpStream) -> Result<(), Error> {
    // 打印客户端连接信息
    println!("收到来自客户端的连接: {}", stream.peer_addr()?);
    let mut data = Vec::new();
    let mut stream = BufReader::new(stream);

    loop {
        data.clear();
        let bytes_read = stream.read_until(b'\n', &mut data)?;

        // 如果读取字节数为0，说明客户端已断开连接
        if bytes_read == 0 {
            println!("客户端断开连接");
            return Ok(());
        }

        // 打印接收到的原始数据用于调试
        println!("接收到数据: {:?}", String::from_utf8_lossy(&data));

        // 尝试解析JSON数据为Point3D结构
        let input: Point3D = match serde_json::from_slice::<Point3D>(&data) {
            Ok(point) => {
                // 验证坐标值是否在合理范围内
                if point.x > 1000000 || point.y > 1000000 || point.z > 1000000 {
                    eprintln!("警告: 坐标值过大，可能导致数值溢出");
                    return write_error_response(&mut stream, "坐标值过大，请使用小于1000000的值");
                }
                point
            },
            Err(e) => {
                eprintln!("JSON解析错误: {}", e);
                return write_error_response(&mut stream, "JSON格式错误，请发送有效的三维点数据");
            }
        };

        // 计算三维点到原点的距离: √(x² + y² + z²)
        // 使用f64类型避免整数溢出
        let value = input.x as f64;
        let distance = (value.powi(2) + (input.y as f64).powi(2) + (input.z as f64).powi(2)).sqrt();

        // 将计算结果发送回客户端
        write!(stream.get_mut(), "{}", distance)?;
        write!(stream.get_mut(), "{}", "\n")?;
        println!("已发送距离计算结果: {}", distance);
    }
}

/// 向客户端发送错误信息
///
/// # 参数
/// - `stream`: 客户端连接流的可变引用
/// - `error_msg`: 要发送的错误信息
///
/// # 返回值
/// - `Result<(), Error>`: IO操作结果
fn write_error_response(stream: &mut BufReader<TcpStream>, error_msg: &str) -> Result<(), Error> {
    let response = format!("错误: {}\n", error_msg);
    stream.get_mut().write_all(response.as_bytes())?;
    eprintln!("已发送错误响应: {}", error_msg);
    Ok(())
}

/// 主函数
///
/// 根据命令行参数启动服务器或客户端
///
/// # 使用方法
/// - 服务器模式: `cargo run -- --server`
/// - 客户端模式: `cargo run -- --client`
///
/// # 功能说明
/// - 服务器模式: 监听8888端口，处理客户端连接并计算三维点距离
/// - 客户端模式: 连接到服务器，允许用户输入三维点坐标并获取距离计算结果
///
/// # 相关文档
/// - 命令行参数: https://doc.rust-lang.org/std/env/index.html
/// - 多线程: https://doc.rust-lang.org/std/thread/index.html
fn main() {
    // 获取命令行参数
    let args: Vec<_> = env::args().collect();

    // 检查参数数量
    if args.len() != 2 {
        eprintln!("使用方法: {} --server | --client", args[0]);
        eprintln!("  --server  : 启动服务器模式");
        eprintln!("  --client  : 启动客户端模式");
        std::process::exit(1);
    }

    // 根据参数选择运行模式
    match args[1].as_str() {
        "--server" => run_server(),
        "--client" => run_client(),
        _ => {
            eprintln!("无效参数: {}", args[1]);
            eprintln!("支持的参数: --server, --client");
            std::process::exit(1);
        }
    }
}

/// 运行服务器模式
///
/// 监听0.0.0.0:8888端口，为每个客户端连接创建新线程处理
///
/// # 功能
/// 1. 绑定到8888端口
/// 2. 监听客户端连接
/// 3. 为每个连接创建新线程
/// 4. 处理JSON格式的三维点数据计算请求
fn run_server() {
    println!("正在启动服务器，监听端口 0.0.0.0:8888");

    // 绑定到指定端口
    let listener = match TcpListener::bind("0.0.0.0:8888") {
        Ok(listener) => {
            println!("服务器启动成功，等待客户端连接...");
            listener
        },
        Err(e) => {
            eprintln!("无法绑定到端口 8888: {}", e);
            eprintln!("请检查端口是否被其他程序占用");
            std::process::exit(1);
        }
    };

    // 接受客户端连接
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("新客户端连接: {}", stream.peer_addr().unwrap_or_else(|_| {
                    // 当无法获取地址时，返回一个默认的SocketAddr
                    "0.0.0.0:0".parse().expect("默认地址应该是有效的")
                }));

                // 为每个客户端连接创建新线程
                thread::spawn(move || {
                    if let Err(error) = handle_client(stream) {
                        eprintln!("客户端连接处理错误: {:?}", error);
                    } else {
                        println!("客户端连接正常结束");
                    }
                });
            },
            Err(e) => {
                eprintln!("接受连接失败: {}", e);
            }
        }
    }
}

/// 运行客户端模式
///
/// 连接到服务器并允许用户输入三维点坐标进行距离计算
///
/// # 功能
/// 1. 连接到本地服务器(127.0.0.1:8888)
/// 2. 接收用户输入的三维点坐标
/// 3. 进行输入验证
/// 4. 发送数据到服务器
/// 5. 接收并显示计算结果
fn run_client() {
    println!("正在连接到服务器 127.0.0.1:8888");

    // 连接到服务器
    let mut stream = match TcpStream::connect("127.0.0.1:8888") {
        Ok(stream) => {
            println!("成功连接到服务器");
            stream
        },
        Err(e) => {
            eprintln!("无法连接到服务器: {}", e);
            eprintln!("请确保服务器正在运行 (cargo run -- --server)");
            std::process::exit(1);
        }
    };

    println!("\n=== 三维点距离计算客户端 ===");
    println!("请输入三维点坐标，用逗号分隔");
    println!("格式: x,y,z (例如: 3,4,5)");
    println!("输入 'quit' 或 'exit' 退出程序");
    println!("================================================");

    loop {
        print!("\n请输入坐标 (x,y,z): ");
        use std::io::Write;
        std::io::stdout().flush().unwrap_or(());

        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();

        // 读取用户输入
        match stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("读取输入失败: {}", e);
                continue;
            }
        }

        // 去除首尾空白字符
        let input = input.trim();

        // 检查退出命令
        if input == "quit" || input == "exit" {
            println!("退出客户端程序");
            break;
        }

        // 检查空输入
        if input.is_empty() {
            println!("输入不能为空，请重新输入");
            continue;
        }

        // 解析输入坐标
        let point = match parse_point_input(input) {
            Ok(point) => point,
            Err(error) => {
                eprintln!("输入错误: {}", error);
                println!("正确格式示例: 3,4,5");
                continue;
            }
        };

        // 序列化并发送数据到服务器
        match serde_json::to_string(&point) {
            Ok(json_str) => {
                if let Err(e) = stream.write_all(json_str.as_bytes()) {
                    eprintln!("发送数据到服务器失败: {}", e);
                    break;
                }
                if let Err(e) = stream.write_all(b"\n") {
                    eprintln!("发送换行符失败: {}", e);
                    break;
                }
            },
            Err(e) => {
                eprintln!("数据序列化失败: {}", e);
                continue;
            }
        }

        // 读取服务器响应
        buffer.clear();
        let mut reader = BufReader::new(&stream);
        match reader.read_until(b'\n', &mut buffer) {
            Ok(0) => {
                eprintln!("服务器连接已断开");
                break;
            },
            Ok(_) => {
                match str::from_utf8(&buffer) {
                    Ok(response) => {
                        let response = response.trim();
                        if response.is_empty() {
                            eprintln!("服务器返回空响应");
                        } else if response.starts_with("错误:") {
                            println!("服务器返回错误: {}", &response[3..]);
                        } else {
                            println!("✅ 原点到点({},{},{})的距离: {:.4}",
                                   point.x, point.y, point.z, response);
                        }
                    },
                    Err(e) => {
                        eprintln!("无法解析服务器响应: {}", e);
                    }
                }
            },
            Err(e) => {
                eprintln!("读取服务器响应失败: {}", e);
                break;
            }
        }
    }
}

/// 解析用户输入的三维点坐标
///
/// # 参数
/// - `input`: 用户输入的字符串，格式应为 "x,y,z"
///
/// # 返回值
/// - `Result<Point3D, String>`: 解析成功返回Point3D，失败返回错误信息
///
/// # 验证项目
/// 1. 格式验证：必须是三个用逗号分隔的数字
/// 2. 数字验证：必须能解析为无符号32位整数
/// 3. 范围验证：数字不能过大或为负数
fn parse_point_input(input: &str) -> Result<Point3D, String> {
    // 按逗号分割输入
    let parts: Vec<&str> = input.split(',').collect();

    // 检查分割后的部分数量
    if parts.len() != 3 {
        return Err(format!(
            "需要3个坐标值，但提供了{}个。正确格式: x,y,z",
            parts.len()
        ).to_string());
    }

    // 解析每个坐标值
    let mut coordinates = Vec::new();
    for (i, part) in parts.iter().enumerate() {
        let trimmed = part.trim();

        // 检查是否为空
        if trimmed.is_empty() {
            return Err(format!("第{}个坐标值为空", i + 1));
        }

        // 尝试解析为数字
        match trimmed.parse::<u32>() {
            Ok(value) => {
                // 检查值是否过大
                if value > 1000000 {
                    return Err(format!(
                        "第{}个坐标值过大 ({}). 请使用小于1,000,000的值",
                        i + 1, value
                    ));
                }
                coordinates.push(value);
            },
            Err(_) => {
                // 检查是否包含非数字字符
                if trimmed.chars().any(|c| c.is_alphabetic()) {
                    return Err(format!(
                        "第{}个坐标值 '{}' 包含字母。坐标值必须是数字",
                        i + 1, trimmed
                    ));
                } else if trimmed.contains('-') {
                    return Err(format!(
                        "第{}个坐标值 '{}' 是负数。请使用正整数",
                        i + 1, trimmed
                    ));
                } else {
                    return Err(format!(
                        "无法解析第{}个坐标值 '{}': 不是有效的正整数",
                        i + 1, trimmed
                    ));
                }
            }
        }
    }

    // 创建并返回Point3D结构
    Ok(Point3D {
        x: coordinates[0],
        y: coordinates[1],
        z: coordinates[2],
    })
}



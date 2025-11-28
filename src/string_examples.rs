// Rust 字符串类型示例：str, &str, String

fn main() {
    println!("=== Rust 字符串类型示例 ===\n");

    // 1. String - 可变、拥有的字符串
    println!("1. String (可变、拥有的字符串):");
    let mut s1 = String::from("Hello");
    println!("  创建: {}", s1);
    s1.push_str(", World!");
    println!("  追加后: {}", s1);
    s1.push('!');
    println!("  再推入字符: {}", s1);
    println!("  长度: {} bytes", s1.len());
    println!("  容量: {} bytes", s1.capacity());

    // 2. &str - 字符串切片 (不可变)
    println!("\n2. &str (字符串切片 - 不可变):");
    let s2: &str = "这是字符串字面量";
    println!("  字符串字面量: {}", s2);

    let s3: &str = &s1; // 从 String 创建 &str
    println!("  从 String 创建: {}", s3);

    // 字符串切片
    let s4: &str = &s1[0..5]; // 切片
    println!("  切片 [0..5]: {}", s4);

    // 3. str (DST - 动态大小类型，通常通过 &str 使用)
    println!("\n3. str (通常通过 &str 使用):");
    let s5: &str = "str 本身是动态大小类型";
    println!("  通过引用使用 str: {}", s5);

    // 4. 类型转换
    println!("\n4. 类型转换:");

    // String -> &str (简单)
    let string_ref: &str = &s1;
    println!("  String -> &str: {}", string_ref);

    // &str -> String (需要分配)
    let str_to_string: String = s2.to_string();
    println!("  &str -> String: {}", str_to_string);

    let str_to_string2: String = s2.to_owned();
    println!("  &str -> String (to_owned): {}", str_to_string2);

    let str_to_string3: String = String::from(s2);
    println!("  &str -> String (String::from): {}", str_to_string3);

    // 5. 函数参数示例
    println!("\n5. 函数参数示例:");
    print_string(&s1); // 传入 &String (自动转为 &str)
    print_string(s2);  // 传入 &str
    print_string("直接传入字面量");

    modify_string(&mut s1); // 传入可变引用
    println!("  修改后的 String: {}", s1);

    // 6. 所有权示例
    println!("\n6. 所有权示例:");
    let s6 = String::from("所有权示例");
    println!("  原字符串: {}", s6);
    take_ownership(s6); // s6 的所有权被转移
    // println!("  这里不能使用 s6: {}", s6); // 这行会编译错误

    let s7 = String::from("借用示例");
    let len = calculate_length(&s7); // 只借用，不转移所有权
    println!("  借用后仍可使用: {}", s7);
    println!("  长度: {}", len);

    // 7. 字符串比较
    println!("\n7. 字符串比较:");
    let s8 = String::from("Hello");
    let s9 = "Hello";
    println!("  String: {} == &str: {} => {}", s8, s9, s8 == s9);

    // 8. Unicode 处理
    println!("\n8. Unicode 处理:");
    let unicode_str = "你好，世界！🌍";
    let unicode_string = String::from("Rust 支持 Unicode");
    println!("  Unicode &str: {}", unicode_str);
    println!("  Unicode String: {}", unicode_string);
    println!("  bytes 长度: {}", unicode_str.len());

    // 9. 字符串格式化
    println!("\n9. 字符串格式化:");
    let name = "Alice";
    let age = 30;
    let formatted = format!("{} 今年 {} 岁", name, age);
    println!("  格式化字符串: {}", formatted);
    let formatted2 = format!("十进制: {}, 十六进制: {:#x}, 二进制: {:#b}", age, age, age);
    println!("  更多格式化: {}", formatted2);

    // 10. 常用操作
    println!("\n10. 常用字符串操作:");
    let text = String::from("   Hello, Rust World!   ");
    println!("  原字符串: '{}'", text);
    println!("  去除空白: '{}'", text.trim());
    println!("  转大写: '{}'", text.to_uppercase());
    println!("  转小写: '{}'", text.to_lowercase());
    println!("  包含 'Rust': {}", text.contains("Rust"));
    println!("  以 'Hello' 开始: {}", text.trim().starts_with("Hello"));

    let words: Vec<&str> = text.trim().split_whitespace().collect();
    println!("  分割后: {:?}", words);

    let replaced = text.replace("World", "编程");
    println!("  替换后: '{}'", replaced.trim());

    // 11. 字符串拼接
    println!("\n11. 字符串拼接:");
    let mut builder = String::new();
    builder.push_str("Hello");
    builder.push(' ');
    builder += "Rust";
    builder += "!";
    println!("  拼接结果: {}", builder);

    // 使用 format! 拼接
    let concatenated = format!("{} {} {}", "使用", "format!", "拼接");
    println!("  format! 拼接: {}", concatenated);
}

// 接受 &str 参数的函数
fn print_string(s: &str) {
    println!("  函数收到字符串: {} (长度: {})", s, s.len());
}

// 接受可变 String 的函数
fn modify_string(s: &mut String) {
    s.push_str(" [已修改]");
}

// 转移所有权的函数
fn take_ownership(s: String) {
    println!("  函数获得所有权: {}", s);
}

// 借用字符串并返回长度
fn calculate_length(s: &String) -> usize {
    s.len()
}
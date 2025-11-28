// Rust 字符串类型对比示例
// str: 动态大小类型，不能直接使用，通常通过 &str 使用
// &str: 不可变字符串切片，通常用于函数参数
// String: 可变、拥有的字符串，堆分配

fn main() {
    println!("=== Rust 字符串类型对比 ===\n");

    // === 基本定义 ===
    println!("1. 基本定义:");

    // 字符串字面量 - 类型是 &'static str
    let literal: &str = "Hello, 世界!";
    println!("   字符串字面量: {} (类型: &'static str)", literal);

    // String - 可变、拥有的字符串
    let owned_string: String = String::from("Hello, 世界!");
    println!("   String 对象: {} (类型: String)", owned_string);

    // 从 String 创建 &str
    let slice_from_string: &str = &owned_string;
    println!("   从 String 创建的切片: {} (类型: &str)", slice_from_string);

    // === 可变性对比 ===
    println!("\n2. 可变性对比:");

    // String 是可变的
    let mut mutable_string = String::from("可变");
    println!("   修改前: {}", mutable_string);
    mutable_string.push_str("字符串");
    println!("   修改后: {}", mutable_string);

    // &str 是不可变的
    let immutable_str = "不可变";
    // immutable_str.push_str("字符串"); // 这行会编译错误
    println!("   &str: {} (不可修改)", immutable_str);

    // === 内存和所有权 ===
    println!("\n3. 内存和所有权:");

    // String 拥有数据，可以转移所有权
    let move_string = String::from("所有权转移");
    println!("   String 转移前: {}", move_string);
    let new_owner = move_string; // 所有权转移
    println!("   String 转移后: {}", new_owner);
    // println!("   原变量: {}", move_string); // 这行会编译错误

    // &str 是借用，不转移所有权
    let borrow_string = String::from("借用不转移所有权");
    let borrowed: &str = &borrow_string;
    println!("   借用前: {}", borrow_string);
    let borrowed_again = borrowed; // 只是复制引用
    println!("   借用后仍可用: {}", borrow_string);
    println!("   复制的引用: {}", borrowed_again);

    // === 函数参数 ===
    println!("\n4. 函数参数使用:");

    let param_string = String::from("函数参数");
    let param_str = "字符串字面量";

    // 函数接受 &str 参数，可以接受所有类型
    process_string(&param_string); // String 自动转换为 &str
    process_string(param_str);      // 直接传入 &str
    process_string("直接传入字面量");

    // === 类型转换方法 ===
    println!("\n5. 类型转换方法:");

    let convert_from = "要从 &str 转换";

    // 多种方式从 &str 创建 String
    let method1: String = convert_from.to_string();
    let method2: String = convert_from.to_owned();
    let method3: String = convert_from.into(); // 需要类型注解
    let method4: String = String::from(convert_from);

    println!("   原始 &str: {}", convert_from);
    println!("   to_string(): {}", method1);
    println!("   to_owned(): {}", method2);
    println!("   into(): {}", method3);
    println!("   String::from(): {}", method4);

    // 从 String 创建 &str
    let string_ref = String::from("从 String 创建 &str");
    let string_slice: &str = &string_ref;
    println!("   String: {}", string_ref);
    println!("   &str: {}", string_slice);

    // === 性能考虑 ===
    println!("\n6. 性能考虑:");

    // 大字符串操作
    let large_string = String::from("这是一个很长的字符串内容...");
    println!("   原始字符串长度: {}", large_string.len());

    // 借用 - 零成本
    let borrowed_large: &str = &large_string;
    println!("   借用字符串长度: {} (零成本)", borrowed_large.len());

    // 克隆 - 需要分配新内存
    let cloned_large: String = large_string.clone();
    println!("   克隆字符串长度: {} (需要内存分配)", cloned_large.len());

    // === 实际使用场景 ===
    println!("\n7. 实际使用场景:");

    // 场景1: 需要修改字符串内容 -> 使用 String
    println!("   场景1 - 需要修改字符串内容:");
    let mut config = String::from("config");
    config.push_str(".json");
    println!("     配置文件名: {}", config);

    // 场景2: 只读取字符串内容 -> 使用 &str
    println!("   场景2 - 只读取字符串内容:");
    validate_username("user123"); // 直接传入字面量

    let username = String::from("admin");
    validate_username(&username); // 从 String 转换

    // 场景3: 函数返回字符串 -> 根据需求选择
    println!("   场景3 - 函数返回字符串:");
    let returned_owned = get_owned_string();
    let returned_borrowed = get_borrowed_string(&username);
    println!("     返回 String: {}", returned_owned);
    println!("     返回 &str: {}", returned_borrowed);
}

// 接受 &str 参数的函数 - 最通用的函数签名
fn process_string(s: &str) {
    println!("   处理字符串: '{}' (长度: {})", s, s.len());
}

// 验证用户名 - 只需要读取，使用 &str
fn validate_username(name: &str) {
    let is_valid = name.len() >= 4 && name.len() <= 20;
    println!("   用户名 '{}' 验证结果: {}", name, if is_valid { "通过" } else { "失败" });
}

// 返回拥有的字符串 - 需要创建新数据时
fn get_owned_string() -> String {
    String::from("这是函数创建的 String")
}

// 返回借用 - 返回现有数据的引用时
fn get_borrowed_string(input: &str) -> &str {
    if input.len() > 5 {
        &input[0..5] // 返回输入的切片
    } else {
        input // 返回整个输入
    }
}
// Rust str 类型详细解释
// str 是动态大小类型 (DST)

fn main() {
    println!("=== Rust str 类型详细解释 ===\n");

    // 1. str 的基本特性
    println!("1. str 的基本特性:");
    println!("   - str 是动态大小类型 (DST)");
    println!("   - 编译时大小未知");
    println!("   - 不能直接创建变量");
    println!("   - 只能通过引用 &str 使用");

    // ❌ 以下代码会编译错误：
    // let my_str: str = "Hello";  // 错误：不能创建 str 变量

    println!("   示例 - 字符串字面量类型:");
    let literal = "Hello, 世界!";
    println!("   字面量 \"Hello, 世界!\" 的实际类型: &'static str");
    println!("   内容: {}", literal);

    // 2. &str 是什么？
    println!("\n2. &str 是什么?");
    println!("   - &str 是对 str 的引用");
    println!("   - 包含指针和长度信息");
    println!("   - 编译时大小已知 (指针 + 长度)");

    demonstrate_str_reference();

    // 3. 不同种类的 &str
    println!("\n3. 不同种类的 &str:");
    demonstrate_str_variants();

    // 4. str 的内部表示
    println!("\n4. str 的内部表示:");
    demonstrate_str_internal();

    // 5. str vs String 的关系
    println!("\n5. str vs String 的关系:");
    demonstrate_str_vs_string();

    // 6. 为什么需要 str？
    println!("\n6. 为什么需要 str:");
    explain_str_purpose();

    // 7. 常见的 str 操作
    println!("\n7. 常见的 str 操作:");
    demonstrate_str_operations();

    // 8. UTF-8 编码
    println!("\n8. UTF-8 编码特性:");
    demonstrate_utf8_str();

    // 9. 函数中使用 str
    println!("\n9. 函数中使用 str:");
    demonstrate_str_in_functions();
}

fn demonstrate_str_reference() {
    let text = "Hello, Rust!";

    println!("   变量 text 的类型: &'static str");
    println!("   内容: {}", text);
    println!("   指向的 str 大小: {} bytes", std::mem::size_of_val(text));
    println!("   &str 引用大小: {} bytes", std::mem::size_of::<&str>());

    // 展示切片也是 &str
    let slice: &str = &text[0..5];
    println!("   切片 [0..5]: {} (类型: &str)", slice);
}

fn demonstrate_str_variants() {
    // 1. &'static str - 程序生命周期内存在
    let static_str: &'static str = "程序生命周期";
    println!("   &'static str: {}", static_str);

    // 2. 从 String 创建的 &str
    let string_var = String::from("动态分配");
    let borrowed_str: &str = &string_var;
    println!("   从 String 借用: {}", borrowed_str);

    // 3. 切片 &str - 注意：不能直接按字节索引 UTF-8 字符串
    let text = "完整的文本";
    // 安全的方法：使用字符边界
    let char_indices: Vec<usize> = text.char_indices().map(|(i, _)| i).collect();
    if char_indices.len() > 4 {
        let slice: &str = &text[..char_indices[4]];
        println!("   前4个字符: {}", slice);
    } else {
        println!("   完整文本: {}", text);
    }

    // 4. 函数参数中的 &str
    take_str_parameter("函数参数");

    // 5. Box<str> - 堆分配的 str
    let boxed_str: Box<str> = "堆分配的str".into();
    let boxed_ref: &str = &boxed_str;
    println!("   Box<str> 引用: {}", boxed_ref);
}

fn demonstrate_str_internal() {
    // str 的内部是 UTF-8 字节序列
    let text = "Hello 世界";
    println!("   文本: {}", text);
    println!("   bytes: {:?}", text.as_bytes());
    println!("   长度 (bytes): {}", text.len());

    // 注意：字符数和字节数可能不同
    println!("   字符数: {}", text.chars().count());
    println!("   字节数: {}", text.len());

    // 展示 str 的字节表示
    println!("   字节序列:");
    for (i, byte) in text.as_bytes().iter().enumerate() {
        println!("     [{}]: 0x{:02x} = {} ({})", i, byte, byte, *byte as char);
    }
}

fn demonstrate_str_vs_string() {
    let string_obj = String::from("这是一个 String");
    let str_slice: &str = &string_obj;

    println!("   String 对象:");
    println!("     值: {}", string_obj);
    println!("     类型: String");
    println!("     栈上大小: {} bytes", std::mem::size_of::<String>());
    println!("     堆上分配: {} bytes", string_obj.capacity());

    println!("   str 切片:");
    println!("     值: {}", str_slice);
    println!("     类型: &str");
    println!("     栈上大小: {} bytes", std::mem::size_of::<&str>());
    println!("     不拥有数据 (借用)");

    // 展示转换
    let from_str = str_slice.to_string();
    println!("   &str -> String: {}", from_str);
}

fn explain_str_purpose() {
    println!("   1. 类型抽象:");
    println!("      - str 代表字符串内容本身");
    println!("      - &str 代表对字符串内容的借用");

    println!("   2. 内存效率:");
    println!("      - 避免不必要的数据复制");
    println!("      - 支持零成本抽象");

    println!("   3. 泛型编程:");
    println!("      - 可以用于函数参数");
    println!("      - 支持多种字符串类型");

    println!("   4. 接口统一:");
    println!("      - String 和字符串字面量都可以转为 &str");
    println!("      - 提供统一的字符串处理接口");

    // 展示泛型使用
    demonstrate_generic_usage();
}

fn demonstrate_str_operations() {
    let text = "Rust 字符串操作";

    // 1. 基本属性
    println!("   原始文本: {}", text);
    println!("   是否为空: {}", text.is_empty());
    println!("   长度 (bytes): {}", text.len());
    println!("   字符数: {}", text.chars().count());

    // 2. 查找操作
    println!("   包含 'Rust': {}", text.contains("Rust"));
    println!("   以 'Rust' 开始: {}", text.starts_with("Rust"));
    println!("   以 '操作' 结束: {}", text.ends_with("操作"));

    // 3. 切片操作 - 使用字符边界进行安全切片
    if let Some(pos) = text.find('字') {
        println!("   '字' 的位置: {}", pos);
        // '字' 之前的部分是安全的，因为 find 返回的是字符边界
        println!("   '字' 之前: {}", &text[..pos]);

        // 找到 '字' 的结束位置
        let char_bounds: Vec<(usize, char)> = text.char_indices().collect();
        if let Some(char_end) = char_bounds.iter().find(|(_, c)| *c == '字').map(|(i, c)| i + c.len_utf8()) {
            println!("   '字' 之后: {}", &text[char_end..]);
        }
    }

    // 4. 字符操作
    println!("   前三个字符: {}", text.chars().take(3).collect::<String>());

    // 5. 行操作
    let multi_line = "第一行\n第二行\n第三行";
    println!("   多行文本行数: {}", multi_line.lines().count());
    for line in multi_line.lines() {
        println!("     行: {}", line);
    }
}

fn demonstrate_utf8_str() {
    // UTF-8 编码的例子
    let unicode_text = "Hi 你好 🌍";

    println!("   Unicode 文本: {}", unicode_text);
    println!("   字节表示: {:?}", unicode_text.as_bytes());
    println!("   总字节数: {}", unicode_text.len());

    // 展示每个字符
    println!("   逐字符分析:");
    for (i, ch) in unicode_text.chars().enumerate() {
        println!("     字符 {}: '{}' (Unicode: U+{:04X})", i, ch, ch as u32);
    }

    // 展示字节边界
    println!("   字节边界:");
    let bytes = unicode_text.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte >= 128 {
            println!("     [{}]: 0x{:02x} (多字节字符部分)", i, byte);
        } else {
            println!("     [{}]: 0x{:02x} = '{}'", i, byte, byte as char);
        }
    }
}

fn demonstrate_str_in_functions() {
    // 函数参数使用 &str
    println!("   函数参数类型:");

    let string_arg = String::from("String 参数");
    let str_arg = "&str 参数";

    process_any_string(&string_arg); // String 自动转为 &str
    process_any_string(str_arg);     // 直接传入 &str
    process_any_string("字面量");    // 字面量也是 &str

    // 函数返回 &str
    let result = get_prefix("Hello, World!", 5);
    println!("   返回的切片: {}", result);

    // 泛型约束
    demonstrate_generic_usage();
}

fn demonstrate_generic_usage() {
    println!("   泛型约束示例:");

    // 这个函数接受任何可以转为 &str 的类型
    let string_val = String::from("泛型测试");
    let str_val = "泛型测试";

    generic_string_processing(&string_val);
    generic_string_processing(str_val);
    generic_string_processing("字面量");
}

// 辅助函数
fn take_str_parameter(s: &str) {
    println!("   函数参数 &str: {}", s);
}

fn process_any_string(s: &str) {
    println!("   处理字符串: '{}' (长度: {})", s, s.len());
}

fn get_prefix(s: &str, n: usize) -> &str {
    if s.len() >= n {
        &s[..n]
    } else {
        s
    }
}

// 泛型函数 - 接受任何可以转为 &str 的类型
fn generic_string_processing<T: AsRef<str>>(text: T) {
    let str_ref = text.as_ref();
    println!("     泛型处理: '{}' (类型: {})", str_ref, std::any::type_name::<T>());
}
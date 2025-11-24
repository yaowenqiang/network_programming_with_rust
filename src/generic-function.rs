use std::ops::Add;

macro_rules! pretty_print {
    ($value:expr) => {
        println!("┌────────────────────────────────────────┐");
        println!("│ 类型: {:30} │", std::any::type_name_of_val(&$value));
        println!("├────────────────────────────────────────┤");
        println!("{:#?}", $value);
        println!("└────────────────────────────────────────┘");
    };
}

macro_rules! debug_struct {
    ($struct:expr) => {{
        let value = &$struct;
        println!("=== 调试信息 ===");
        println!("变量: {}", stringify!($struct));
        println!("类型: {}", std::any::type_name_of_val(value));
        println!("内存地址: {:p}", value);
        println!("值: {:#?}", value);
        println!("=== 结束 ===");
        value
    }};
}


/// 漂亮的彩色打印宏
macro_rules! pp {
    ($value:expr) => {{
        let val = &$value;
        let name = stringify!($value);
        let type_name = std::any::type_name_of_val(val);
        
        // 计算合适的边框宽度
        let name_width = name.chars().count();
        let type_width = type_name.chars().count();
        let max_width = name_width.max(type_width).max(30) + 10;
        
        let border = "─".repeat(max_width);
        
        println!("{BRIGHT_CYAN}┌{}┐{RESET}", border);
        println!("{BRIGHT_CYAN}│{RESET} {YELLOW}{}{RESET} {:width$} {BRIGHT_CYAN}│{RESET}", 
            "变量:", name, width = max_width - 6 - name_width);
        println!("{BRIGHT_CYAN}│{RESET} {GREEN}{}{RESET} {:width$} {BRIGHT_CYAN}│{RESET}", 
            "类型:", type_name, width = max_width - 6 - type_width);
        println!("{BRIGHT_CYAN}├{}┤{RESET}", border);
        println!("{BRIGHT_GREEN}值:{RESET}");
        println!("{BRIGHT_WHITE}{:#?}{RESET}", val);
        println!("{BRIGHT_CYAN}└{}┘{RESET}", border);
        println!(); // 空行分隔
    }};
    
    ($value:expr, $label:literal) => {{
        let val = &$value;
        let label = $label;
        
        println!("{BRIGHT_MAGENTA}┌────────────────────────────────────────────────────┐{RESET}");
        println!("{BRIGHT_MAGENTA}│{RESET} {BRIGHT_YELLOW}🏷️ {label:<47}{BRIGHT_MAGENTA}│{RESET}");
        println!("{BRIGHT_MAGENTA}│{RESET} {CYAN}表达式:{RESET} {YELLOW}{:<38}{BRIGHT_MAGENTA}│{RESET}", stringify!($value));
        println!("{BRIGHT_MAGENTA}│{RESET} {CYAN}类型:{RESET} {GREEN}{:<40}{BRIGHT_MAGENTA}│{RESET}", std::any::type_name_of_val(val));
        println!("{BRIGHT_MAGENTA}├────────────────────────────────────────────────────┤{RESET}");
        println!("{BRIGHT_GREEN}值:{RESET}");
        println!("{BRIGHT_WHITE}{:#?}{RESET}", val);
        println!("{BRIGHT_MAGENTA}└────────────────────────────────────────────────────┘{RESET}");
        println!();
    }};
    
    // 简洁版本，只有一行
    ($value:expr, short) => {{
        let val = &$value;
        println!("{YELLOW}{}{RESET} = {GREEN}{}{RESET} {CYAN}(类型: {}){RESET}", 
            stringify!($value), 
            val, 
            std::any::type_name_of_val(val)
        );
    }};
}

/// 彩色打印多个变量
macro_rules! pp_multiple {
    ($($value:expr),* $(,)?) => {
        println!("{BRIGHT_BLUE}┌────────────────────────────────────────────────────┐{RESET}");
        println!("{BRIGHT_BLUE}│{RESET} {BRIGHT_WHITE}🔍 多个变量调试信息{RESET} {:28} {BRIGHT_BLUE}│{RESET}", "");
        println!("{BRIGHT_BLUE}├────────────────────────────────────────────────────┤{RESET}");
        $(
            println!("{BRIGHT_BLUE}│{RESET} {YELLOW}{:<20}{RESET} {GREEN}{:<25}{RESET} {BRIGHT_BLUE}│{RESET}", 
                stringify!($value), 
                std::any::type_name_of_val(&$value)
            );
        )*
        println!("{BRIGHT_BLUE}├────────────────────────────────────────────────────┤{RESET}");
        $(
            println!("{BRIGHT_BLUE}│{RESET} {YELLOW}{:<20}{RESET} {WHITE}{:?}{RESET} {BRIGHT_BLUE}│{RESET}", 
                stringify!($value), 
                &$value
            );
        )*
        println!("{BRIGHT_BLUE}└────────────────────────────────────────────────────┘{RESET}");
        println!();
    };
}

/// 带成功/错误状态的打印
macro_rules! pp_status {
    ($value:expr, $status:expr) => {{
        let val = &$value;
        let status_color = if $status { BRIGHT_GREEN } else { BRIGHT_RED };
        let status_icon = if $status { "✓" } else { "✗" };
        
        println!("{BRIGHT_CYAN}┌────────────────────────────────────────────────────┐{RESET}");
        println!("{BRIGHT_CYAN}│{RESET} {}{} {}{:45} {BRIGHT_CYAN}│{RESET}", 
            status_color, status_icon, stringify!($value), "");
        println!("{BRIGHT_CYAN}│{RESET} {CYAN}类型:{RESET} {GREEN}{:<40}{BRIGHT_CYAN}│{RESET}", 
            std::any::type_name_of_val(val));
        println!("{BRIGHT_CYAN}├────────────────────────────────────────────────────┤{RESET}");
        println!("{BRIGHT_WHITE}{:#?}{RESET}", val);
        println!("{BRIGHT_CYAN}└────────────────────────────────────────────────────┘{RESET}");
        println!();
    }};
}


 // 定义 ANSI 颜色代码
macro_rules! define_colors {
    () => {
        pub const RESET: &str = "\x1b[0m";
        pub const BOLD: &str = "\x1b[1m";
        pub const RED: &str = "\x1b[31m";
        pub const GREEN: &str = "\x1b[32m";
        pub const YELLOW: &str = "\x1b[33m";
        pub const BLUE: &str = "\x1b[34m";
        pub const MAGENTA: &str = "\x1b[35m";
        pub const CYAN: &str = "\x1b[36m";
        pub const WHITE: &str = "\x1b[37m";
        pub const BRIGHT_RED: &str = "\x1b[91m";
        pub const BRIGHT_GREEN: &str = "\x1b[92m";
        pub const BRIGHT_YELLOW: &str = "\x1b[93m";
        pub const BRIGHT_BLUE: &str = "\x1b[94m";
        pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
        pub const BRIGHT_CYAN: &str = "\x1b[96m";
        pub const BRIGHT_WHITE: &str = "\x1b[97m";
        pub const BG_BLACK: &str = "\x1b[40m";
        pub const BG_BLUE: &str = "\x1b[44m";
    };
}

define_colors!();

/// 提取文件名（去掉路径）
fn get_short_filename(file: &str) -> &str {
    file.rsplit('/').next().unwrap_or(file)
        .rsplit('\\').next().unwrap_or(file)
}

/// 漂亮的彩色打印宏，包含文件名和行号
macro_rules! pp {
    ($value:expr) => {{
        let val = &$value;
        let name = stringify!($value);
        let type_name = std::any::type_name_of_val(val);
        let file = get_short_filename(file!());
        let line = line!();
        let column = column!();
        
        // 计算合适的边框宽度
        let name_width = name.chars().count();
        let type_width = type_name.chars().count();
        let file_info_width = format!("{}:{}:{}", file, line, column).chars().count();
        let max_width = name_width.max(type_width).max(file_info_width).max(30) + 10;
        
        let border = "─".repeat(max_width);
        
        println!("{BRIGHT_CYAN}┌{}┐{RESET}", border);
        println!("{BRIGHT_CYAN}│{RESET} {YELLOW}📍 {file}:{line}:{column}{RESET} {:width$} {BRIGHT_CYAN}│{RESET}", 
            "", width = max_width - 10 - file_info_width);
        println!("{BRIGHT_CYAN}│{RESET} {YELLOW}🔤 变量:{RESET} {BRIGHT_WHITE}{name}{RESET} {:width$} {BRIGHT_CYAN}│{RESET}", 
            "", width = max_width - 8 - name_width);
        println!("{BRIGHT_CYAN}│{RESET} {GREEN}📊 类型:{RESET} {BRIGHT_WHITE}{type_name}{RESET} {:width$} {BRIGHT_CYAN}│{RESET}", 
            "", width = max_width - 8 - type_width);
        println!("{BRIGHT_CYAN}├{}┤{RESET}", border);
        println!("{BRIGHT_GREEN}📋 值:{RESET}");
        println!("{BRIGHT_WHITE}{:#?}{RESET}", val);
        println!("{BRIGHT_CYAN}└{}┘{RESET}", border);
        println!(); // 空行分隔
    }};
    
    ($value:expr, $label:literal) => {{
        let val = &$value;
        let label = $label;
        let file = get_short_filename(file!());
        let line = line!();
        let column = column!();
        
        println!("{BRIGHT_MAGENTA}┌────────────────────────────────────────────────────┐{RESET}");
        println!("{BRIGHT_MAGENTA}│{RESET} {BRIGHT_YELLOW}🏷️ {label:<47}{BRIGHT_MAGENTA}│{RESET}");
        println!("{BRIGHT_MAGENTA}│{RESET} {CYAN}📍 位置:{RESET} {YELLOW}{file}:{line}:{column}{RESET} {:20} {BRIGHT_MAGENTA}│{RESET}", "");
        println!("{BRIGHT_MAGENTA}│{RESET} {CYAN}🔤 表达式:{RESET} {YELLOW}{:<38}{BRIGHT_MAGENTA}│{RESET}", stringify!($value));
        println!("{BRIGHT_MAGENTA}│{RESET} {CYAN}📊 类型:{RESET} {GREEN}{:<40}{BRIGHT_MAGENTA}│{RESET}", std::any::type_name_of_val(val));
        println!("{BRIGHT_MAGENTA}├────────────────────────────────────────────────────┤{RESET}");
        println!("{BRIGHT_GREEN}📋 值:{RESET}");
        println!("{BRIGHT_WHITE}{:#?}{RESET}", val);
        println!("{BRIGHT_MAGENTA}└────────────────────────────────────────────────────┘{RESET}");
        println!();
    }};
    
    // 简洁版本，只有一行
    ($value:expr, short) => {{
        let val = &$value;
        let file = get_short_filename(file!());
        let line = line!();
        println!("{YELLOW}[{file}:{line}] {}{RESET} = {GREEN}{:?}{RESET} {CYAN}(类型: {}){RESET}", 
            stringify!($value), 
            val, 
            std::any::type_name_of_val(val)
        );
    }};
    
    // 极简版本，只有值和位置
    ($value:expr, minimal) => {{
        let val = &$value;
        let file = get_short_filename(file!());
        let line = line!();
        println!("{BRIGHT_BLUE}[{file}:{line}]{RESET} {BRIGHT_WHITE}{:?}{RESET}", val);
    }};
}

/// 彩色打印多个变量，包含位置信息
macro_rules! pp_multiple {
    ($($value:expr),* $(,)?) => {{
        let file = get_short_filename(file!());
        let line = line!();
        let column = column!();
        
        println!("{BRIGHT_BLUE}┌────────────────────────────────────────────────────┐{RESET}");
        println!("{BRIGHT_BLUE}│{RESET} {BRIGHT_WHITE}🔍 多个变量调试信息{RESET} {:28} {BRIGHT_BLUE}│{RESET}", "");
        println!("{BRIGHT_BLUE}│{RESET} {CYAN}📍 位置:{RESET} {YELLOW}{file}:{line}:{column}{RESET} {:20} {BRIGHT_BLUE}│{RESET}", "");
        println!("{BRIGHT_BLUE}├────────────────────────────────────────────────────┤{RESET}");
        $(
            println!("{BRIGHT_BLUE}│{RESET} {YELLOW}{:<20}{RESET} {GREEN}{:<25}{RESET} {BRIGHT_BLUE}│{RESET}", 
                stringify!($value), 
                std::any::type_name_of_val(&$value)
            );
        )*
        println!("{BRIGHT_BLUE}├────────────────────────────────────────────────────┤{RESET}");
        $(
            println!("{BRIGHT_BLUE}│{RESET} {YELLOW}{:<20}{RESET} {WHITE}{:?}{RESET} {BRIGHT_BLUE}│{RESET}", 
                stringify!($value), 
                &$value
            );
        )*
        println!("{BRIGHT_BLUE}└────────────────────────────────────────────────────┘{RESET}");
        println!();
    }};
}

/// 带成功/错误状态的打印
macro_rules! pp_status {
    ($value:expr, $status:expr) => {{
        let val = &$value;
        let file = get_short_filename(file!());
        let line = line!();
        let column = column!();
        let status_color = if $status { BRIGHT_GREEN } else { BRIGHT_RED };
        let status_icon = if $status { "✅" } else { "❌" };
        
        println!("{BRIGHT_CYAN}┌────────────────────────────────────────────────────┐{RESET}");
        println!("{BRIGHT_CYAN}│{RESET} {}{} {}{:45} {BRIGHT_CYAN}│{RESET}", 
            status_color, status_icon, stringify!($value), "");
        println!("{BRIGHT_CYAN}│{RESET} {CYAN}📍 位置:{RESET} {YELLOW}{file}:{line}:{column}{RESET} {:20} {BRIGHT_CYAN}│{RESET}", "");
        println!("{BRIGHT_CYAN}│{RESET} {CYAN}📊 类型:{RESET} {GREEN}{:<40}{BRIGHT_CYAN}│{RESET}", 
            std::any::type_name_of_val(val));
        println!("{BRIGHT_CYAN}├────────────────────────────────────────────────────┤{RESET}");
        println!("{BRIGHT_WHITE}{:#?}{RESET}", val);
        println!("{BRIGHT_CYAN}└────────────────────────────────────────────────────┘{RESET}");
        println!();
    }};
}

/// 内存信息版本
macro_rules! pp_memory {
    ($value:expr) => {{
        let val = &$value;
        let size = std::mem::size_of_val(val);
        let align = std::mem::align_of_val(val);
        let ptr = val as *const _ as usize;
        let file = get_short_filename(file!());
        let line = line!();
        let column = column!();
        
        println!("{BRIGHT_YELLOW}┌────────────────────────────────────────────────────┐{RESET}");
        println!("{BRIGHT_YELLOW}│{RESET} {BRIGHT_WHITE}🧠 内存信息 - {}{RESET} {:26} {BRIGHT_YELLOW}│{RESET}", stringify!($value), "");
        println!("{BRIGHT_YELLOW}│{RESET} {CYAN}📍 位置:{RESET} {YELLOW}{file}:{line}:{column}{RESET} {:20} {BRIGHT_YELLOW}│{RESET}", "");
        println!("{BRIGHT_YELLOW}│{RESET} {CYAN}📊 类型:{RESET} {GREEN}{:<40}{BRIGHT_YELLOW}│{RESET}", std::any::type_name_of_val(val));
        println!("{BRIGHT_YELLOW}│{RESET} {CYAN}⚖️  大小:{RESET} {WHITE}{:<6} 字节{RESET} {:28} {BRIGHT_YELLOW}│{RESET}", size, "");
        println!("{BRIGHT_YELLOW}│{RESET} {CYAN}📐 对齐:{RESET} {WHITE}{:<6} 字节{RESET} {:28} {BRIGHT_YELLOW}│{RESET}", align, "");
        println!("{BRIGHT_YELLOW}│{RESET} {CYAN}🏠 地址:{RESET} {MAGENTA}0x{:016x}{RESET} {:20} {BRIGHT_YELLOW}│{RESET}", ptr);
        println!("{BRIGHT_YELLOW}├────────────────────────────────────────────────────┤{RESET}");
        println!("{BRIGHT_GREEN}📋 值:{RESET}");
        println!("{BRIGHT_WHITE}{:#?}{RESET}", val);
        println!("{BRIGHT_YELLOW}└────────────────────────────────────────────────────┘{RESET}");
        println!();
    }};
}

/// 性能计时版本
macro_rules! pp_time {
    ($label:literal, $code:block) => {{
        let file = get_short_filename(file!());
        let line = line!();
        let start = std::time::Instant::now();
        let result = $code;
        let duration = start.elapsed();
        
        println!("{BRIGHT_GREEN}┌────────────────────────────────────────────────────┐{RESET}");
        println!("{BRIGHT_GREEN}│{RESET} {BRIGHT_WHITE}⏱️  计时: {}{RESET} {:30} {BRIGHT_GREEN}│{RESET}", $label, "");
        println!("{BRIGHT_GREEN}│{RESET} {CYAN}📍 位置:{RESET} {YELLOW}{file}:{line}{RESET} {:28} {BRIGHT_GREEN}│{RESET}", "");
        println!("{BRIGHT_GREEN}│{RESET} {CYAN}⏰ 耗时:{RESET} {BRIGHT_WHITE}{:.6} 秒 ({:.3} 毫秒){RESET} {:12} {BRIGHT_GREEN}│{RESET}", 
            duration.as_secs_f64(), duration.as_secs_f64() * 1000.0);
        println!("{BRIGHT_GREEN}└────────────────────────────────────────────────────┘{RESET}");
        println!();
        result
    }};
}

/// 条件调试版本（只在调试模式显示）
macro_rules! debug_pp {
    ($($arg:tt)*) => {{
        if cfg!(debug_assertions) {
            pp!($($arg)*);
        }
    }};
}



#[derive(Debug)]
struct Tuple<T> {
    first: T,
    second: T,
}

fn main() {
    let tuple_u32: Tuple<u32> = Tuple{ first:4u32, second: 2u32 };
    let tuple_u64: Tuple<u64> = Tuple{ first:4u64, second: 6u64 };

    println!("{tuple_u32:?}");
    println!("{tuple_u64:?}");

    let tuple: Tuple<String> = Tuple {first: "One".to_owned(), second:"Two".to_owned()};
    let tuple2 = Tuple {first: "One", second:"Two"};
    //println!("{}", sum(tuple));
    println!("{:#?}", tuple);
    println!("{:#?}", tuple2);
    debug_struct!(tuple);
    debug_struct!(tuple2);

    pretty_print!(tuple);
    pretty_print!(tuple2);

    //assert_eq!(tuple, tuple2);

    let a = "hello";
    let b = a.to_owned();
    let c = String::from("world");
    let d = 1;
    let e = d.to_owned();

    pretty_print!(a);
    pretty_print!(b);
    pretty_print!(c);
    pretty_print!(d);
    pretty_print!(e);

    pp!(a);
    pp!(b);
    pp!(c);
    pp!(d);
    pp!(e);
}

fn sum<T: Add<Output = T>>(tuple: Tuple<T>) ->T {
    tuple.first + tuple.second
}

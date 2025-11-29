// Rust GDB 调试示例程序

use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: Option<String>,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        println!("创建 Person: {}", name);
        Person {
            name: name.to_string(),
            age,
            email: None,
        }
    }

    fn set_email(&mut self, email: &str) {
        self.email = Some(email.to_string());
    }

    fn greet(&self) {
        println!("你好，我是 {}，今年 {} 岁", self.name, self.age);
        if let Some(ref email) = self.email {
            println!("邮箱: {}", email);
        }
    }

    fn calculate_birth_year(&self, current_year: u32) -> u32 {
        current_year - self.age
    }
}

fn fibonacci(n: u32) -> u64 {
    println!("计算 fibonacci({})", n);
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn bubble_sort(arr: &mut [i32]) {
    println!("冒泡排序开始: {:?}", arr);
    let n = arr.len();

    for i in 0..n {
        println!("外层循环 i = {}", i);
        for j in 0..n - i - 1 {
            println!("  比较 arr[{}] = {} 和 arr[{}] = {}", j, arr[j], j + 1, arr[j + 1]);
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                println!("  交换后: {:?}", arr);
            }
        }
    }

    println!("冒泡排序结束: {:?}", arr);
}

fn process_numbers(numbers: Vec<i32>) -> Vec<i32> {
    println!("处理数字: {:?}", numbers);
    let mut result = Vec::new();

    for num in numbers {
        println!("处理数字: {}", num);
        let processed = if num % 2 == 0 {
            num * 2
        } else {
            num * 3 + 1
        };
        result.push(processed);
        println!("处理后: {}", processed);
    }

    result
}

fn simulate_delay(millis: u64) {
    println!("等待 {} 毫秒...", millis);
    thread::sleep(Duration::from_millis(millis));
    println!("等待完成");
}

fn cause_panic() {
    println!("即将引发 panic");
    let v = vec![1, 2, 3];
    println!("访问越界索引: {}", v[10]); // 这会 panic
}

fn main() {
    println!("=== Rust GDB 调试示例程序 ===\n");

    // 1. 结构体调试
    println!("1. 结构体示例:");
    let mut person1 = Person::new("张三", 25);
    person1.set_email("zhangsan@example.com");
    person1.greet();

    let birth_year = person1.calculate_birth_year(2024);
    println!("出生年份: {}", birth_year);

    // 2. 函数调用和递归
    println!("\n2. 递归函数示例:");
    let fib_result = fibonacci(5);
    println!("Fibonacci(5) = {}", fib_result);

    // 3. 算法调试
    println!("\n3. 排序算法示例:");
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    bubble_sort(&mut numbers);
    println!("最终排序结果: {:?}", numbers);

    // 4. 向量和迭代
    println!("\n4. 向量处理示例:");
    let input_numbers = vec![1, 2, 3, 4, 5];
    let processed = process_numbers(input_numbers);
    println!("处理结果: {:?}", processed);

    // 5. 线程和延迟
    println!("\n5. 线程延迟示例:");
    simulate_delay(100);

    // 6. 错误处理 - 这会 panic
    println!("\n6. 错误处理示例:");
    println!("准备引发 panic...");

    // 如果不想让程序 panic，可以注释掉这行
    // cause_panic();

    // 7. 指针和引用
    println!("\n7. 指针和引用示例:");
    let x = 42;
    let y = &x;
    let z = Box::new(x);

    println!("x = {} (值)", x);
    println!("y = {} (引用)", y);
    println!("z = {} (Box 指针)", z);

    // 8. 字符串处理
    println!("\n8. 字符串处理示例:");
    let mut text = String::from("Hello, Rust!");
    text.push_str(" GDB 调试");

    let slice = &text[0..12];
    println!("原始字符串: {}", text);
    println!("字符串切片: {}", slice);

    println!("\n程序执行完成！");
}
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

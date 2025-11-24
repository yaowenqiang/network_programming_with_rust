fn main() {
    let borrowed_str = "hello";
    let owned_string = borrowed_str.to_owned();
    print!("{owned_string}");
    // &str → String
    let string_literal: &str = "hello";
    let owned_string: String = string_literal.to_owned();

    // 等同于
    let s1: String = "hello".to_owned();
    let s2: String = "hello".to_string();
    let s3: String = String::from("hello");
}

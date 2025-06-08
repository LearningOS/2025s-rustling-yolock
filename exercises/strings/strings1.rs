// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    // 正确将字符串字面量转换为 String 类型
    // Rust 中 "blue" 是 &str 类型，需通过 to_string() 方法转换
    "blue".to_string()
}

fn main() {
    let s = String::from("Hello Rust!");

    // 不可變引用 (immutable reference)
    let len = calculate_length(&s);  
    println!("字串: \"{}\" 的長度是 {}", s, len);

    // 可變引用 (mutable reference)
    let mut s2 = String::from("Hello");
    change(&mut s2);  
    println!("修改後的字串: {}", s2);
}

// 函數參數用 & 表示借用 (不可變借用)
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 可變引用，用 &mut
fn change(s: &mut String) {
    s.push_str(", world!");
}

// 列舉
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    println!("📚 Rust 數據類型展示");

    // 整數類型
    let int_num: i32 = -42;
    let uint_num: u32 = 42;
    println!("整數: int_num = {}, uint_num = {}", int_num, uint_num);

    // 浮點數
    let float_num: f64 = 3.14159;
    println!("浮點數: float_num = {}", float_num);

    // 布林值
    let is_rust_fun: bool = true;
    println!("布林值: is_rust_fun = {}", is_rust_fun);

    // 字元 (單一 Unicode 字元)
    let letter: char = '🐷';
    println!("字元: letter = {}", letter);

    // 字串 (String vs &str)
    let str_slice: &str = "Hello";
    let string_obj: String = String::from("World");
    println!("字串: {} {}", str_slice, string_obj);

    // 陣列
    let arr: [i32; 3] = [1, 2, 3];
    println!("陣列: arr = {:?}", arr);

    // Tuple
    let tup: (i32, f64, &str) = (500, 6.4, "tuple!");
    println!("Tuple: {:?}", tup);

    // 常量
    const MAX_POINTS: u32 = 100_000;
    println!("常量: MAX_POINTS = {}", MAX_POINTS);

    // 可變變量
    let mut score = 0;
    score += 10;
    println!("可變變量: score = {}", score);

    // 結構體
    struct Person {
        name: String,
        age: u8,
    }
    let user = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("結構體: {} is {} years old", user.name, user.age);

    // 列舉 - 用掉所有變體
    let directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    for dir in directions {
        match dir {
            Direction::Up => println!("列舉: 向上 ⬆️"),
            Direction::Down => println!("列舉: 向下 ⬇️"),
            Direction::Left => println!("列舉: 向左 ⬅️"),
            Direction::Right => println!("列舉: 向右 ➡️"),
        }
    }
}

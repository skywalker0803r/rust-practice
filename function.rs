fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }
    max
}

fn main() {
    let numbers = vec![10, 25, 7, 42, 15];
    let chars = vec!['a', 'x', 'm', 'z'];

    let max_num = largest(&numbers);
    let max_char = largest(&chars);

    println!("最大數字是 {}", max_num);
    println!("最大字元是 {}", max_char);
}

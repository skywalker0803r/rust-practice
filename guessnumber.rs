use std::io;
use rand::Rng; // 需要在 Cargo.toml 加上 rand = "0.8"

// 定義常量：猜數字的最小值與最大值
const MIN: u32 = 1;
const MAX: u32 = 100;

fn main() {
    println!("🎲 幸福豬娛樂城猜數字遊戲 🎲");
    println!("請猜一個 {MIN} 到 {MAX} 之間的數字");

    // 使用常量產生隨機數
    let secret_number = rand::thread_rng().gen_range(MIN..=MAX);
    let mut attempts = 0;

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("讀取失敗");

        // 將輸入字串轉成數字
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("請輸入有效的數字！");
                continue;
            }
        };

        attempts += 1;

        if guess < secret_number {
            println!("太小了！⬆️");
        } else if guess > secret_number {
            println!("太大了！⬇️");
        } else {
            println!("🎉 猜對了！答案是 {secret_number}，你總共猜了 {attempts} 次！");
            break;
        }
    }
}

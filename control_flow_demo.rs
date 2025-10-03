fn main() {
    // 這是一個單行註解：宣告變數 x
    let x = 10;

    /* 這是一個多行註解
       範例程式會展示不同的控制流語法 */
    println!("開始控制流示範，x = {}", x);

    // if-else 條件判斷
    if x < 5 {
        println!("x 小於 5");
    } else if x == 10 {
        println!("x 等於 10");
    } else {
        println!("x 大於 5 且不等於 10");
    }

    // loop 無限迴圈，內部用 break 停止
    let mut count = 0;
    loop {
        count += 1;
        println!("loop count = {}", count);
        if count == 3 {
            break; // 跳出 loop
        }
    }

    // while 迴圈
    let mut n = 3;
    while n > 0 {
        println!("while n = {}", n);
        n -= 1;
    }

    // for 迴圈，用於迭代範圍
    for i in 1..=5 { // 1 到 5，包含 5
        println!("for i = {}", i);
    }

    println!("控制流示範結束");
}

// src/main.rs

// ownership example program
// Demonstrates: move, borrow (&), mutable borrow (&mut), clone(), Copy types

fn main() {
    println!("== Ownership example (run with `cargo run`) ==");

    // 1) Move: ownership transfers when returning from get_scores()
    let scores = get_scores();
    println!("scores from get_scores(): {:?}", scores);

    // 2) Passing ownership into a function consumes it (move)
    let total = calc_score_take_ownership(scores);
    println!("total from calc_score_take_ownership: {}", total);

    // The following line would not compile if uncommented, because `scores` was moved above:
    // println!("scores after move: {:?}", scores);

    // 3) Borrowing: pass a reference so ownership is not moved
    let mut scores2 = vec![10, 20, 30];
    let total2 = calc_score_borrow(&scores2);
    println!("total from calc_score_borrow: {} (scores2 still usable: {:?})", total2, scores2);

    // 4) Mutable borrow: allow the called function to modify the value
    modify_scores(&mut scores2);
    println!("scores2 after modify_scores (mut borrow): {:?}", scores2);

    // 5) Clone: explicitly duplicate the data if you need another owner
    let scores3 = vec![100, 200];
    let scores3_clone = scores3.clone();
    // Both can be used because we cloned
    println!("scores3: {:?}, scores3_clone: {:?}", scores3, scores3_clone);

    // 6) Copy trait types (primitive types) are copied, not moved
    let a = 5;
    let b = a; // i32 implements Copy, so `a` is still valid
    println!("a: {}, b: {} (ints are Copy)", a, b);

    // 7) Borrow rules demo (immutable borrows can be many)
    let book = String::from("Rust 所有權練習");
    let r1 = &book;
    let r2 = &book; // multiple immutable borrows allowed
    println!("book r1: {}, r2: {}", r1, r2);

    // 8) Mutable borrow exclusivity (only one mutable borrow at a time)
    let mut book2 = String::from("Mutable book");
    {
        let mr = &mut book2;
        mr.push_str(" (edited)");
        println!("book2 inside mutable borrow: {}", mr);
    } // mr goes out of scope here; we can borrow again
    println!("book2 after mutable borrow block: {}", book2);

    // If you try to create a mutable borrow while immutable borrows are still in scope,
    // the compiler will reject it. Example (won't compile):
    // let r3 = &book2;
    // let mr2 = &mut book2; // error: cannot borrow as mutable because it's also borrowed as immutable

    println!("== End of demo ==");
}

fn get_scores() -> Vec<i32> {
    let scores = vec![1450, 9527, 5566];
    // ownership of `scores` moves to the caller
    scores
}

fn calc_score_take_ownership(scores: Vec<i32>) -> i32 {
    // `scores` is owned by this function; it will be dropped at the end
    let mut total = 0;
    for s in scores.iter() {
        total += s;
    }
    total
}

fn calc_score_borrow(scores: &Vec<i32>) -> i32 {
    // borrow: this function only has an immutable reference
    let mut total = 0;
    for s in scores.iter() {
        total += s;
    }
    total
}

fn modify_scores(scores: &mut Vec<i32>) {
    // mutable borrow: this function can change the caller's Vec
    scores.push(999);
}
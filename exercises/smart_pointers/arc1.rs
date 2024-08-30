// arc1.rs
//
// 在这个练习中，我们有一个名为 "numbers" 的 u32 向量，其值范围从 0 到 99 -- [ 0, 1, 2, ..., 98, 99 ]。
// 我们希望在 8 个不同的线程中同时使用这组数字。每个线程将获取每隔第八个值的和，并带有偏移量。
//
// 第一个线程（偏移 0），将求和 0, 8, 16, ...
// 第二个线程（偏移 1），将求和 1, 9, 17, ...
// 第三个线程（偏移 2），将求和 2, 10, 18, ...
// ...
// 第八个线程（偏移 7），将求和 7, 15, 23, ...
//
// 由于我们使用的是线程，我们的值需要是线程安全的。因此，我们使用 Arc。我们需要在两个 TODO 处进行修改。
//
// 通过在第一个 TODO 注释处为 `shared_numbers` 填入一个值，并在第二个 TODO 注释处为 `child_numbers` 创建一个初始绑定，使这段代码编译通过。
// 尽量不要创建 `numbers` 向量的任何副本！
//
// 执行 `rustlings hint arc1` 或使用 `hint` 观察子命令以获取提示。

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers); // TODO
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers); // TODO
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}

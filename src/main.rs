/*

Rust 中最简单的错误处理方式就是使用 panic。它会打印出一条错误信息并打印出栈调用情况，最终结束当前线程:

若 panic 发生在 main 线程，那程序会随之退出
如果是在生成的( spawn )子线程中发生 panic, 那么当前的线程会结束，但是程序依然会继续运行
*/
// 填空
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // 实现下面的代码
        __
    }

    println!("Exercise Failed if printing out this line!");
}

fn main() {
    drink(__);

    println!("Exercise Failed if printing out this line!");
}

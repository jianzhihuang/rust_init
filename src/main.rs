/*


// 引用的生命周期是 'static :
let s: &'static str = "hello world";

// 'static 也可以用于特征约束中:
fn generic<T>(x: T) where T: 'static {}
虽然它们都是 'static，但是也稍有不同。
*/

/* 使用两种方法填空 */
fn main() {
    __;
    need_static(v);

    println!("Success!")
}

fn need_static(r: &'static str) {
    assert_eq!(r, "hello");
}

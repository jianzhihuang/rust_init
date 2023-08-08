/*&'static
作为一个引用生命周期，&'static 说明该引用指向的数据可以跟程序活得一样久，但是该引用的生命周期依然有可能被强转为一个更短的生命周期。
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

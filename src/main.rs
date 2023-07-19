// // 填空并修复错误
// // 1. 不要使用 `to_string()`
// // 2. 不要添加/删除任何代码行
// fn main() {
//     let mut s = "hello, ".to_owned();
//     s.push_str("world");
//     s.push('!');

//     move_ownership(s.clone());

//     assert_eq!(s, "hello, world!");

//     println!("Success!")
// }

// fn move_ownership(s: String) {
//     println!("ownership of \"{}\" is moved here!", s)
// }
// 填空
fn main() {
    let mut s = String::from("hello, world");

    // let slice1: &str = &s;
    // 使用两种方法
    let slice1: &str = s.as_str();
    assert_eq!(slice1, "hello, world");

    let slice2 = slice1;
    assert_eq!(slice2, "hello");

    let mut slice3: String = s.clone().to_owned();
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    println!("Success!")
}

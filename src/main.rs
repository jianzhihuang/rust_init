/*
Result<T> 是一个枚举类型用于描述返回的结果或错误，它包含两个成员(变体 variants) :

Ok(T): 返回一个结果值 T
Err(e): 返回一个错误，e 是具体的错误值
简而言之，如果期待一个正确的结果，就返回 Ok，反之则是 Err。
*/

// 填空并修复错误
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> __ {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn main() {
    let result = multiply("10", "2");
    assert_eq!(result, __);

    let result = multiply("t", "2");
    assert_eq!(result.__, 8);

    println!("Success!")
}

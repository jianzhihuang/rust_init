/*Rust 的借用检查器使用显式的生命周期标注来确定一个引用的合法范围。但是对于用户来说，我们在大多数场景下，都无需手动去标注生命周期，原因是编译器会在某些情况下自动应用生命周期消除规则。

在了解编译器使用哪些规则帮我们消除生命周期之前，首先还是需要知道该如何手动标记生命周期。

函数
大家先忽略生命周期消除规则，让我们看看，函数签名中的生命周期有哪些限制:

需要为每个引用标注上合适的生命周期
返回值中的引用，它的生命周期要么跟某个引用参数相同，要么是 'static
*/

// fn main() {
//     {
//         let r; // ---------+-- 'a
//                //          |

//         //          |
//         let x = 5; // -+-- 'b  |
//         r = &x; //  |       |
//                 // -+       |
//                 //          |
//         println!("r: {}", r); //          |
//     } // ---------+
// }

/* 添加合适的生命周期标注，让下面的代码工作 */
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {}

/* 使用三种方法修复下面的错误  */
// fn invalid_output<'a>() -> &'a String {
//     &String::from("foo")
// }

// fn main() {}
// fn invalid_output() -> Option<String> {
//     Some(String::from("foo"))
// }

// fn main() {}
// fn invalid_output() -> String {
//     String::from("foo")
// }fn main() {}
// fn invalid_output<'a>(s: &'a String) -> &'a String {
//     s
// }

// fn main() {}
// `print_refs` 有两个引用参数，它们的生命周期 `'a` 和 `'b` 至少得跟函数活得一样久
// fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
//     println!("x is {} and y is {}", x, y);
// }

// /* 让下面的代码工作 */
// fn failed_borrow<'a>() {
//     let _x = 12;

//     // ERROR: `_x` 活得不够久does not live long enough
//     let y = &_x;

//     // 在函数内使用 `'a` 将会报错，原因是 `&_x` 的生命周期显然比 `'a` 要小
//     // 你不能将一个小的生命周期强转成大的
// }

// fn main() {
//     let (four, nine) = (4, 9);

//     print_refs(&four, &nine);
//     // 这里，four 和 nice 的生命周期必须要比函数 print_refs 长

//     failed_borrow();
//     // `failed_borrow`  没有传入任何引用去限制生命周期 `'a`，因此，此时的 `'a` 生命周期是没有任何限制的，它默认是 `'static`
// }
/* 增加合适的生命周期标准，让代码工作 */

// `i32` 的引用必须比 `Borrowed` 活得更久
// #[derive(Debug)]
// struct Borrowed<'a>(&'a i32);

// // 类似的，下面两个引用也必须比结构体 `NamedBorrowed` 活得更久
// #[derive(Debug)]
// struct NamedBorrowed<'a> {
//     x: &'a i32,
//     y: &'a i32,
// }

// #[derive(Debug)]
// enum Either<'a> {
//     Num(i32),
//     Ref(&'a i32),
// }

// fn main() {
//     let x = 18;
//     let y = 15;

//     let single = Borrowed(&x);
//     let double = NamedBorrowed { x: &x, y: &y };
//     let reference = Either::Ref(&x);
//     let number = Either::Num(y);

//     println!("x is borrowed in {:?}", single);
//     println!("x and y are borrowed in {:?}", double);
//     println!("x is borrowed in {:?}", reference);
//     println!("y is *not* borrowed in {:?}", number);
// }

/* 让代码工作 */

// #[derive(Debug)]
// struct NoCopyType {}

// #[derive(Debug)]
// struct Example<'a, 'b> {
//     a: &'a u32,
//     b: &'b NoCopyType,
// }

// fn main() {
//     let var_a = 35;
//     let example: Example;

//     {
//         let var_b: &'static NoCopyType = &NoCopyType {};

//         /* 修复错误 */
//         example = Example {
//             a: &var_a,
//             b: &var_b,
//         };
//     }

//     println!("(Success!) {:?}", example);
// }
/* 添加合适的生命周期让下面代码工作 */
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// impl<'a> ImportantExcerpt<'a> {
//     fn level(&'a self) -> i32 {
//         3
//     }
// }

// fn main() {}
/* 移除所有可以消除的生命周期标注 */

fn nput(x: &i32) {
    println!("`annotated_input`: {}", x);
}

fn pass(x: &i32) -> &i32 {
    x
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    x
}

struct Owner(i32);

impl Owner {
    fn add_one(&mut self) {
        self.0 += 1;
    }
    fn print(&self) {
        println!("`print`: {}", self.0);
    }
}

struct Person {
    age: u8,
    name: str,
}

enum Either {
    Num(i32),
    Ref(i32),
}

fn main() {}

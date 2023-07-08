// pub struct MinSlice<T, const N: usize> {
//     pub head: [T; N],
//     pub tail: [T],
// }
// impl<T, const N: usize> MinSlice<T, N> {
//     pub fn from_slice(slice: &[T]) -> Option<Self> {
//         // 实现从切片构建 MinSlice 的逻辑
//     }
// }
// fn main() {
//     let slice: &[u8] = b"Hello, world";
//     let reference: Option<&u8> = slice.get(6);
//     // 我们知道 `.get` 返回的是 `Some(b' ')`
//     // 但编译器不知道
//     assert!(reference.is_some());

//     let slice: &[u8] = b"Hello, world";

//     // 当编译构建 MinSlice 时会进行长度检查，也就是在编译期我们就知道它的长度是 12
//     // 在运行期，一旦 `unwrap` 成功，在 `MinSlice` 的作用域内，就再无需任何检查
//     let minslice = MinSlice::<u8, 12>::from_slice(slice).unwrap();
//     let value: u8 = minslice.head[6];
//     assert_eq!(value, b' ')
// }

// // 修复错误
// struct Array<T, const N: usize> {
//     data: [T; N],
// }

// fn main() {
//     let arrays = [
//         Array { data: [1, 2, 3] },
//         Array {
//             data: [1.0 as i32, 2.0 as i32, 3.0 as i32],
//         },
//         Array { data: [1, 2, 3] },
//     ];
// }

// 填空

// fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
//     println!("{:?}", arr);
// }
// fn main() {
//     let arr = [1, 2, 3];
//     print_array(arr);

//     let arr = ["hello", "world"];
//     print_array(arr);
// }
// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]

// fn check_size<T>(val: T)
// where
//     Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
// {
//     //...
// }

// // 修复 main 函数中的错误
// fn main() {
//     check_size([0u8; 767]);
//     check_size([0i32; 191]);
//     check_size(["hello你好"; 7]); // size of &str ?
//     check_size([(); 1].map(|_| "hello你好".to_string())); // size of String?
//     check_size(['中'; 1]); // size of char ?
// }

// pub enum Assert<const CHECK: bool> {}

// pub trait IsTrue {}

// impl IsTrue for Assert<true> {}

// struct Sheep {
//     naked: bool,
//     name: String,
// }

// impl Sheep {
//     fn is_naked(&self) -> bool {
//         self.naked
//     }

//     fn shear(&mut self) {
//         if self.is_naked() {
//             // `Sheep` 结构体上定义的方法可以调用 `Sheep` 所实现的特征的方法
//             println!("{} is already naked...", self.name());
//         } else {
//             println!("{} gets a haircut!", self.name);

//             self.naked = true;
//         }
//     }
// }

// trait Animal {
//     // 关联函数签名；`Self` 指代实现者的类型
//     // 例如我们在为 Pig 类型实现特征时，那 `new` 函数就会返回一个 `Pig` 类型的实例，这里的 `Self` 指代的就是 `Pig` 类型
//     fn new(name: String) -> Self;

//     // 方法签名
//     fn name(&self) -> String;

//     fn noise(&self) -> String;

//     // 方法还能提供默认的定义实现
//     fn talk(&self) {
//         println!("{} says {}", self.name(), self.noise());
//     }
// }

// impl Animal for Sheep {
//     // `Self` 被替换成具体的实现者类型： `Sheep`
//     fn new(name: String) -> Sheep {
//         Sheep {
//             name: name,
//             naked: false,
//         }
//     }

//     fn name(&self) -> String {
//         self.name.clone()
//     }

//     fn noise(&self) -> String {
//         if self.is_naked() {
//             "baaaaah?".to_string()
//         } else {
//             "baaaaah!".to_string()
//         }
//     }

//     // 默认的特征方法可以被重写
//     fn talk(&self) {
//         println!("{} pauses briefly... {}", self.name, self.noise());
//     }
// }

// fn main() {
//     // 这里的类型注释时必须的
//     let mut dolly: Sheep = Animal::new("Dolly".to_string());
//     // TODO ^ 尝试去除类型注释，看看会发生什么

//     dolly.talk();
//     dolly.shear();
//     dolly.talk();
// }

// 完成两个 `impl` 语句块
// 不要修改 `main` 中的代码
// trait Hello {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }

//     fn say_something(&self) -> String;
// }

// struct Student {}
// impl Hello for Student {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }
//     fn say_something(&self) -> String {
//         String::from("I'm a good student")
//     }
// }
// struct Teacher {}
// impl Hello for Teacher {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }
//     fn say_something(&self) -> String {
//         String::from("I'm not a bad teacher")
//     }
// }

// fn main() {
//     let s = Student {};
//     assert_eq!(s.say_hi(), "hi");
//     assert_eq!(s.say_something(), "I'm a good student");

//     let t = Teacher {};
//     assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
//     assert_eq!(t.say_something(), "I'm not a bad teacher");

//     println!("Success!")
// }

// `Centimeters`, 一个元组结构体，可以被比较大小
// #[derive(PartialEq, PartialOrd)]
// struct Centimeters(f64);

// // `Inches`, 一个元组结构体可以被打印
// #[derive(Debug)]
// struct Inches(i32);

// impl Inches {
//     fn to_centimeters(&self) -> Centimeters {
//         let &Inches(inches) = self;

//         Centimeters(inches as f64 * 2.54)
//     }
// }

// // 添加一些属性让代码工作
// // 不要修改其它代码！
// #[derive(Debug, PartialEq, PartialOrd)]
// struct Seconds(i32);

// fn main() {
//     let _one_second = Seconds(1);

//     println!("One second looks like: {:?}", _one_second);
//     let _this_is_true = _one_second == _one_second;
//     let _this_is_true = _one_second > _one_second;

//     let foot = Inches(12);

//     println!("One foot equals {:?}", foot);

//     let meter = Centimeters(100.0);

//     let cmp = if foot.to_centimeters() < meter {
//         "smaller"
//     } else {
//         "bigger"
//     };

//     println!("One foot is {} than one meter.", cmp);
// }

// use std::ops::Mul;

// // 实现 fn multiply 方法
// // 如上所述，`+` 需要 `T` 类型实现 `std::ops::Add` 特征
// // 那么, `*` 运算符需要实现什么特征呢? 你可以在这里找到答案: https://doc.rust-lang.org/core/ops/
// fn multiply<T>(x: T, y: T) -> T
// where
//     T: Mul<Output = T> + Copy,
// {
//     x * y
// }

// fn main() {
//     assert_eq!(6, multiply(2u8, 3u8));
//     assert_eq!(5.0, multiply(1.0, 5.0));

//     println!("Success!")
// }

// 修复错误，不要修改 `main` 中的代码!
// use std::ops;

// struct Foo;
// struct Bar;
// #[derive(PartialEq, Debug)]
// struct FooBar;
// #[derive(PartialEq, Debug)]
// struct BarFoo;

// // 下面的代码实现了自定义类型的相加： Foo + Bar = FooBar
// impl ops::Add<Bar> for Foo {
//     type Output = FooBar;

//     fn add(self, _rhs: Bar) -> FooBar {
//         FooBar
//     }
// }

// impl ops::Sub<Bar> for Foo {
//     type Output = BarFoo;

//     fn sub(self, _rhs: Bar) -> BarFoo {
//         BarFoo
//     }
// }

// fn main() {
//     // 不要修改下面代码
//     // 你需要为 FooBar 派生一些特征来让代码工作
//     assert_eq!(Foo + Bar, FooBar);
//     assert_eq!(Foo - Bar, BarFoo);

//     println!("Success!")
// }

// 实现 `fn summary`
// 修复错误且不要移除任何代码行
// trait Summary {
//     fn summarize(&self) -> String;
// }

// #[derive(Debug)]
// struct Post {
//     title: String,
//     author: String,
//     content: String,
// }

// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("The author of post {} is {}", self.title, self.author)
//     }
// }

// #[derive(Debug)]
// struct Weibo {
//     username: String,
//     content: String,
// }

// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{} published a weibo {}", self.username, self.content)
//     }
// }

// fn main() {
//     let post = Post {
//         title: "Popular Rust".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust is awesome!".to_string(),
//     };
//     let weibo = Weibo {
//         username: "sunface".to_string(),
//         content: "Weibo seems to be worse than Tweet".to_string(),
//     };

//     summary(&post);
//     summary(&weibo);

//     println!("{:?}", post);
//     println!("{:?}", weibo);
// }

// // 在下面实现 `fn summary` 函数
// fn summary(item: &impl Summary) {
//     println!("{}", item.summarize());
// }

// struct Sheep {}
// struct Cow {}

// trait Animal {
//     fn noise(&self) -> String;
// }

// impl Animal for Sheep {
//     fn noise(&self) -> String {
//         "baaaaah!".to_string()
//     }
// }

// impl Animal for Cow {
//     fn noise(&self) -> String {
//         "moooooo!".to_string()
//     }
// }

// // 返回一个类型，该类型实现了 Animal 特征，但是我们并不能在编译期获知具体返回了哪个类型
// // 修复这里的错误，你可以使用虚假的随机，也可以使用特征对象
// fn random_animal(random_number: f64) -> Box<dyn Animal> {
//     if random_number < 0.5 {
//         Box::new(Sheep {})
//     } else {
//         Box::new(Cow {})
//     }
// }

// fn main() {
//     let random_number = 0.234;
//     let animal = random_animal(random_number);
//     println!(
//         "You've randomly chosen an animal, and it says {}",
//         animal.noise()
//     );
// }
// fn main() {
//     assert_eq!(sum(1, 2), 3);
// }

// // 通过两种方法使用特征约束来实现 `fn sum`
// fn sum<T>(x: T, y: T) -> T
// where
//     T: std::ops::Add<Output = T>,
// {
//     x + y
// }
// // 修复代码中的错误
// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }

// impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {:?}", self.x);
//         } else {
//             println!("The largest member is y = {:?}", self.y);
//         }
//     }
// }
// #[derive(Debug, PartialEq, PartialOrd)]
// struct Unit(i32);

// fn main() {
//     let pair = Pair {
//         x: Unit(1),
//         y: Unit(3),
//     };

//     pair.cmp_display();
// }

// 填空
fn example1() {
    // `T: Trait` 是最常使用的方式
    // `T: Fn(u32) -> u32` 说明 `T` 只能接收闭包类型的参数
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }

    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    let mut cacher = Cacher::new(|x| x + 1);
    assert_eq!(cacher.value(10), __);
    assert_eq!(cacher.value(15), __);
}

fn example2() {
    // 还可以使用 `where` 来约束 T
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    let mut cacher = Cacher::new(|x| x + 1);
    assert_eq!(cacher.value(20), __);
    assert_eq!(cacher.value(25), __);
}

fn main() {
    example1();
    example2();

    println!("Success!")
}

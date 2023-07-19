// use std::ops::Sub;

// #[derive(Debug, PartialEq)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// // 用三种方法填空: 其中两种使用默认的泛型参数，另外一种不使用
// impl<T: Sub<Output = T>> Sub for Point<T> {
//     type Output = Point<T>;

//     fn sub(self, other: Self) -> Self::Output {
//         Point {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }

// fn main() {
//     assert_eq!(
//         Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
//         Point { x: 1, y: 3 }
//     );

//     println!("Success!")
// }
// trait UsernameWidget {
//     fn get(&self) -> String;
// }

// trait AgeWidget {
//     fn get(&self) -> u8;
// }

// struct Form {
//     username: String,
//     age: u8,
// }

// impl UsernameWidget for Form {
//     fn get(&self) -> String {
//         self.username.clone()
//     }
// }

// impl AgeWidget for Form {
//     fn get(&self) -> u8 {
//         self.age
//     }
// }

// fn main() {
//     let form = Form {
//         username: "rustacean".to_owned(),
//         age: 28,
//     };

//     // 如果你反注释下面一行代码，将看到一个错误: Fully Qualified Syntax
//     // 毕竟，这里有好几个同名的 `get` 方法
//     //
//     // println!("{}", form.get());

//     let username = UsernameWidget::get(&form);
//     assert_eq!("rustacean".to_owned(), username);
//     let age = AgeWidget::get(&form); // 你还可以使用以下语法 `<Form as AgeWidget>::get`
//     assert_eq!(28, age);

//     println!("Success!")
// }
///////////////
///
trait Pilot {
    fn fly(&self) -> String;
}

trait Wizard {
    fn fly(&self) -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) -> String {
        String::from("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("Up!")
    }
}

impl Human {
    fn fly(&self) -> String {
        String::from("*waving arms furiously*")
    }
}

fn main() {
    let person = Human;

    assert_eq!(
        (<Human as Pilot>::fly)(&person),
        "This is your captain speaking."
    );
    assert_eq!(Wizard::fly(&person), "Up!");

    assert_eq!(Human::fly(&person), "*waving arms furiously*");

    println!("Success!")
}
/////////////////////////

// trait Person {
//     fn name(&self) -> String;
// }

// Person 是 Student 的 supertrait .
// 实现 Student 需要同时实现 Person.
// trait Student: Person {
//     fn university(&self) -> String;
// }

// trait Programmer {
//     fn fav_language(&self) -> String;
// }

// // CompSciStudent (computer science student) 是 Programmer
// // 和 Student 的 subtrait. 实现 CompSciStudent 需要先实现这两个 supertraits.
// trait CompSciStudent: Programmer + Student {
//     fn git_username(&self) -> String;
// }

// fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
//     format!(
//         "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
//         student.name(),
//         student.university(),
//         student.fav_language(),
//         student.git_username()
//     )
// }

// struct CSStudent {
//     name: String,
//     university: String,
//     fav_language: String,
//     git_username: String,
// }

// // 为 CSStudent 实现所需的特征
// impl CompSciStudent for CSStudent {
//     fn git_username(&self) -> String {
//         self.git_username.clone()
//     }
// }

// impl Student for CSStudent {
//     fn university(&self) -> String {
//         self.university.clone()
//     }
// }

// impl Programmer for CSStudent {
//     fn fav_language(&self) -> String {
//         self.fav_language.clone()
//     }
// }

// impl Person for CSStudent {
//     fn name(&self) -> String {
//         self.name.clone()
//     }
// }
// fn main() {
//     let student = CSStudent {
//         name: "Sunfei".to_string(),
//         university: "XXX".to_string(),
//         fav_language: "Rust".to_string(),
//         git_username: "sunface".to_string(),
//     };

//     // 填空
//     println!("{}", comp_sci_student_greeting(&student));
// }

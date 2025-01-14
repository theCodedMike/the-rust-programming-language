#![allow(dead_code)]
#![allow(unused_variables)]

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // 常量，且不可变

static mut REQUEST_LIMIT: usize = 1000; // 静态变量 是可变的，也就是static后面可以加 mut

/// 3.1 变量和可变性
///
/// cargo r --bin var-and-mut
///
/// ## 目录
/// ### 常量
///
/// ### 遮蔽
///
fn main() {
    /* 变量和可变性 */
    let mut x = 5; // 如果想要改变变量的值，必须加mut关键字
    println!("The value of x is: {}", x); // 5
    x = 6;
    println!("The value of x is: {}", x); // 6
    println!();

    /* 常量, 不允许使用 mut */
    println!("{}", THREE_HOURS_IN_SECONDS); // 10800
    println!();

    /* 遮蔽 */
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x); // 12
    }
    println!("The value of x is: {}", x); // 6
    println!();

    /* 变量解构 */
    let (a, mut b) = (true, false);
    println!("a = {}, b = {}", a, b); // a = true, b = false
    b = true;
    assert_eq!(a, b);
    println!();

    /* 解构式赋值 */
    let (a, b, c, d, e) = (1, 2_u32, 3.2, 4.8_f32, 'a');
    println!("{}, {}, {}, {}, {}", a, b, c, d, e); // 1, 2, 3.2, 4.8, a
    let c: i32;
    let d: i32;
    [c, .., d, _] = [1, 2, 3, 4, 5]; // _表示匹配1个值, ..表示范围range
    println!("c: {}, d: {}", c, d); // c: 1, d: 4
    for i in 1..5 {
        println!("{}", i);
    }
    // 1
    // 2
    // 3
    // 4
    let Student { name, age } = Student {
        name: "Neo".to_string(),
        age: 24,
    };
    println!("name: {}, age: {}", name, age); // name: Neo, age: 24
    println!();

    /* 变量如果未初始化，则编译器不会初始化它(不像Java)，需要手动初始化后才能使用它 */
    let default_value = 0;
    // 必须初始化，否则编译器报错 error[E0381]: used binding `default_value` isn't initialized
    println!("default_value is {}", default_value); // 0
    println!();

    /* 补充 */
    // 静态变量
    // 缺点是不支持函数，即 static mut COMPANY_NAME = String::from("111"); ， 仅支持一些字面量，
    //对于线程安全的全局变量，见 16_fearless_concurrency
    println!("{}", add_one());
}

struct Student {
    name: String,
    age: usize,
}

fn add_one() -> usize {
    unsafe {
        REQUEST_LIMIT += 1;
        REQUEST_LIMIT
    }
}

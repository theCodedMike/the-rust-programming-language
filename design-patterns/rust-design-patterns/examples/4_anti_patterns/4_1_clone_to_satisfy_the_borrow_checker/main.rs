#![allow(unused_mut)]
//! ## 用Clone满足借用检查器
//!
//! ### 描述
//! 借用检查阻止了Rust用户开发不安全的代码，以此保证：只存在一个可变引用，或者（许多）不可变引用
//!
//! 如果编写的代码不符合这些条件，而开发者通过克隆变量来解决编译器错误，就会产生这种反模式
//!
//! ### 出发点
//! 使用.clone()会导致数据被复制。两者之间的任何变化都不会同步，除了Rc<T>、Arc<T>
//!
//! 一般来说，应该经过深思熟虑，充分了解其后果再clone
//!
//! ### 优点
//!
//! ### 缺点
//!
//! ### 讨论
//!
//! ### 执行
//! cargo r --example 4_1
//!

fn main() {
    // 定义任意变量
    let mut x = 5;

    // 借用 `x`（先clone）
    let y = &mut (x.clone());

    // 由于 x.clone(), x 并未被借用, 这行代码可以运行。
    println!("{}", x); // 5
    println!("{}", y); // 5
                       // 用这个借用做点什么，防止因Rust优化直接砍掉这个借用
    *y += 1;

    println!("{}", x); // 5
    println!("{}", y); // 6
}
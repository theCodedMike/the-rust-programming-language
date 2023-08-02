/// 16.4 使用 Sync 和 Send 特征的可扩展并发
///
/// cargo r --bin sync-and-send
///
/// ## 目录:
/// ### 通过 Send 允许在线程间转移所有权
/// - Send trait 表明类型的所有权可以在线程间传递
///
/// ### Sync 允许多线程访问
/// - Sync trait 表明可以在多个线程中安全地拥有该类型的引用
/// - 对于任意类型 T，如果 &T是 Send 的话 T 就是 Sync 的
///
/// ### 手动实现 Send 和 Sync 是不安全的
///
fn main() {
    // 通过 Send 允许在线程间转移所有权
    // Sync 允许多线程访问
    // 手动实现 Send 和 Sync 是不安全的
}

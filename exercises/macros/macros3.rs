// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


// 定义宏
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

mod macros {
    // 模块内部也可以使用外部定义的宏
}

fn main() {
    // 在 main 函数中调用宏
    my_macro!();
}

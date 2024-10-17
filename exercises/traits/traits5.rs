// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.

pub trait SomeTrait {
    fn some_function(&self) -> bool;
}

pub trait OtherTrait {
    fn other_function(&self) -> bool;
}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {
    fn some_function(&self) -> bool {
        true
    }
}

impl OtherTrait for SomeStruct {
    fn other_function(&self) -> bool {
        true
    }
}

impl SomeTrait for OtherStruct {
    fn some_function(&self) -> bool {
        true
    }
}

impl OtherTrait for OtherStruct {
    fn other_function(&self) -> bool {
        true
    }
}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    let some_struct = SomeStruct {};
    let other_struct = OtherStruct {};
    println!("{}", some_func(some_struct)); // true
    println!("{}", some_func(other_struct)); // true
}

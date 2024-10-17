// from_into.rs
//
// The From trait is used for value-to-value conversions. If From is implemented
// correctly for a type, the Into trait should work conversely. You can read
// more about it at https://doc.rust-lang.org/std/convert/trait.From.html
//
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a
// hint.

use std::ptr::NonNull;

struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node {
            val,
            next: None,
        }
    }

    // 示例函数，展示如何安全地处理原始指针
    unsafe fn process(&self) {
        let next_node_ptr = self.next.as_ref();
        if let Some(next_node) = next_node_ptr {
            // 正确地解引用并访问 val 字段
            let _next_val = &(*next_node.as_ptr()).val; // 使用引用
        }
    }

    // 如果 T 实现了 Clone，你可以使用这个辅助函数来克隆值
    fn clone_val(&self, val: &T) -> T 
    where
        T: Clone,
    {
        val.clone()
    }
}

fn main() {
    let node = Node::new(10);
    unsafe {
        node.process();
    }
}
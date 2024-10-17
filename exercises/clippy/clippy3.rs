// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.


fn main() {
    // 移除这个不必要的 Option，因为它永远不会被用来包含值
    // let my_option: Option<()> = None;
    // if my_option.is_none() {
    //     my_option.unwrap(); // 这将导致 panic，因为 None 没有值可以解包
    // }

    let my_arr = [
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 使用 clear 方法来清空向量，而不是 resize
    let mut my_vec = vec![1, 2, 3, 4, 5];
    my_vec.clear();
    println!("This Vec is empty, see? {:?}", my_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 正确的交换两个变量的值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}

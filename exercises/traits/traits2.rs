// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.

trait AppendBar {
    fn append_bar(self) -> Self;
}

// 为 Vec<String> 实现 AppendBar trait
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        self.into_iter().map(|s| s + "Bar").collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("FooBar"));
        assert!(foo.is_empty());
    }
}

fn main() {
    // Example usage
    let vec_of_strings = vec![String::from("Hello"), String::from("World")];
    let appended_bar = vec_of_strings.append_bar();
    for s in appended_bar {
        println!("{}", s);
    }
}

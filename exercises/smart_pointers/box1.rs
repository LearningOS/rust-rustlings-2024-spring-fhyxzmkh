// box1.rs
//
// 在编译时，Rust 需要知道一个类型占多少空间。这对于递归类型来说是有问题的，
// 因为一个值可以包含另一个相同类型的值。为了解决这个问题，我们可以使用 `Box` -
// 一个用于在堆上存储数据的智能指针，它也允许我们包装递归类型。
//
// 在这个练习中，我们将实现递归类型 `cons list` - 一种在函数式编程语言中常见的数据结构。
// 每个 cons list 中的项包含两个元素：当前项的值和下一项。最后一项是一个称为 `Nil` 的值。
//
// 第一步：在枚举定义中使用 `Box` 以使代码编译通过
// 第二步：通过替换 `todo!()` 创建空的和非空的 cons list
//
// 注意：不应更改测试
//
// 执行 `rustlings hint box1` 或使用 `hint` 观察子命令以获取提示。

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("这是一个空的 cons list: {:?}", create_empty_list());
    println!("这是一个非空的 cons list: {:?}", create_non_empty_list());
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Nil))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}

// cow1.rs
//
// 这个练习探索了 Cow（Clone-On-Write 类型）。Cow 是一个克隆-写时复制智能指针。
// 它可以封装并提供对借用数据的不可变访问，并在需要修改或所有权时延迟克隆数据。
// 该类型设计用于通过 Borrow trait 处理一般借用数据。
//
// 这个练习旨在向你展示在将数据传递给 Cow 时的预期行为。
// 通过在 TODO 标记处检查 Cow::Owned(_) 和 Cow::Borrowed(_) 来修复单元测试。
//
// 执行 `rustlings hint cow1` 或使用 `hint` 观察子命令以获取提示。

use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // 如果尚未拥有，则克隆到向量中。
            input.to_mut()[i] = -v;
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // 因为需要修改 `input`，所以发生克隆。
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // 不需要修改 `input`，所以不发生克隆。
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Borrowed(_) => Ok(()),
            _ => Err("Expected borrowed value"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // 我们也可以不使用 `&` 传递 `slice`，这样 Cow 直接拥有它。
        // 在这种情况下，不发生修改，因此也不发生克隆，但结果仍然是拥有的，因为从未借用或修改过。
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // 当然，如果发生修改，这也是如此。在这种情况下，对 `to_mut()` 的调用返回对之前相同数据的引用。
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }
}
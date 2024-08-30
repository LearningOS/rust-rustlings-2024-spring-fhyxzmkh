// tests9.rs
//
// Rust 能够与 C/C++ 和其他静态编译语言共享 FFI 接口，甚至可以在代码内部进行链接！它通过 extern 块实现这一点，就像下面的代码一样。
//
// `extern` 关键字后面的短字符串表示外部导入的函数将遵循哪个 ABI。在这个练习中，使用 "Rust"，而其他变体如 "C" 表示标准 C ABI，"stdcall" 表示 Windows ABI。
//
// 外部导入的函数在 extern 块中声明，用分号标记签名结束，而不是用花括号。可以对这些函数声明应用一些属性来修改链接行为，例如 #[link_name = ".."] 来修改实际的符号名称。
//
// 如果你想将你的符号导出到链接环境中，可以在函数定义前用相同的 ABI 字符串标记 `extern` 关键字。Rust 函数的默认 ABI 实际上是 "Rust"，所以如果你想链接纯 Rust 函数，整个 extern 术语可以省略。
//
// Rust 默认会对符号进行修饰，就像 C++ 一样。为了抑制这种行为并使这些函数可以通过名称访问，可以应用 #[no_mangle] 属性。
//
// 在这个练习中，你的任务是使测试用例能够调用模块 Foo 中的 `my_demo_function`。`my_demo_function_alias` 是 `my_demo_function` 的别名，所以测试用例中的两行代码应该调用同一个函数。
//
// 除了添加两行属性外，你不应该修改任何现有代码。

extern "Rust" {
    
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // No `extern` equals `extern "Rust"`.
    #[no_mangle]
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}

// iterators2.rs
//
// 在这个练习中，你将学习到迭代器的一些独特优势。按照步骤完成练习。
//
// 执行 `rustlings hint iterators2` 或使用 `hint` 观察子命令以获取提示。

// 第一步。
// 完成 `capitalize_first` 函数。
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

// 第二步。
// 将 `capitalize_first` 函数应用于字符串切片的切片。
// 返回一个字符串向量。
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|&word| capitalize_first(word)).collect()
}

// 第三步。
// 再次将 `capitalize_first` 函数应用于字符串切片的切片。
// 返回一个字符串。
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words
        .iter()
        .map(|&word| capitalize_first(word))
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}

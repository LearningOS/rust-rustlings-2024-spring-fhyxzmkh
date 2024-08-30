// from_str.rs
//
// 这与 from_into.rs 类似，但这次我们将实现 `FromStr` 并返回错误，而不是回退到默认值。
// 此外，在实现 FromStr 之后，你可以使用字符串上的 `parse` 方法生成实现类型的对象。
// 你可以在 https://doc.rust-lang.org/std/str/trait.FromStr.html 阅读更多相关信息。
//
// 执行 `rustlings hint from_str` 或使用 `hint` 观察子命令以获取提示。

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// 我们将使用这个错误类型来实现 `FromStr`。
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // 空输入字符串
    Empty,
    // 字段数量不正确
    BadLen,
    // 名字字段为空
    NoName,
    // 从 parse::<usize>() 包装的错误
    ParseInt(ParseIntError),
}

// 步骤：
// 1. 如果提供的字符串长度为 0，应返回一个错误
// 2. 根据字符串中存在的逗号将其分割
// 3. 分割操作应只返回 2 个元素，否则返回一个错误
// 4. 提取分割操作的第一个元素并将其用作名字
// 5. 提取分割操作的另一个元素并将其解析为 `usize` 类型的年龄，例如 `"4".parse::<usize>()`
// 6. 如果在提取名字和年龄时出现问题，应返回一个错误
// 如果一切顺利，则返回一个 Person 对象的 Result
//
// 顺便说一下：`Box<dyn Error>` 实现了 `From<&'_ str>`。这意味着如果你想返回一个字符串错误消息，
// 你可以通过 `return Err("my error message".into())` 来实现。

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty() {
            Err(ParsePersonError::Empty)
        } else {
            let info: Vec<&str> = s.split(',').collect();
            if info.len() != 2 {
                Err(ParsePersonError::BadLen)
            } else {
                if info[0].is_empty() {
                    Err(ParsePersonError::NoName)
                } else {
                    match info[1].parse::<usize>() {
                        Ok(age) => Ok(Person {
                            name: info[0].to_string(),
                            age,
                        }),
                        Err(e) => Err(ParsePersonError::ParseInt(e)),
                    }
                }
            }
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}

// from_into.rs
//
// From trait 用于值到值的转换。如果为某个类型正确实现了 From trait，
// 那么 Into trait 应该能够相反地工作。你可以在
// https://doc.rust-lang.org/std/convert/trait.From.html 阅读更多相关信息。
//
// 执行 `rustlings hint from_into` 或使用 `hint` 观察子命令以获取提示。

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// 我们实现了 Default trait 以便在提供的字符串无法转换为 Person 对象时
// 使用默认值作为后备
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// 你的任务是完成这个实现，以便 `let p = Person::from("Mark,20")` 能够编译。
// 请注意，你需要将年龄部分解析为 `usize`，例如使用 `"4".parse::<usize>()`。
// 解析结果需要适当处理。
//
// 步骤：
// 1. 如果提供的字符串长度为 0，则返回 Person 的默认值。
// 2. 将给定的字符串按逗号分隔。
// 3. 提取分隔操作的第一个元素并将其作为名字。
// 4. 如果名字为空，则返回 Person 的默认值。
// 5. 提取分隔操作的另一个元素并将其解析为 `usize` 类型的年龄。
// 如果在解析年龄时出现问题，则返回 Person 的默认值。
// 否则，返回一个实例化的 Person 对象，包含解析结果。

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        if s.is_empty() {
            Person::default()
        } else {
            let info: Vec<&str> = s.split(',').collect();

            if info[0].is_empty() || info.len() != 2 {
                Person::default()
            } else {
                match info[1].parse::<usize>() {
                    Ok(age) => Person {
                        name: info[0].to_string(),
                        age,
                    },
                    Err(_) => Person::default(),
                }
            }
        }
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an
        // error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}

// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn trim_me(input: &str) -> String {
    // trim() 方法去除字符串两端的空白字符
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // 方法1：使用 format! 宏
    format!("{} world!", input)

    // 方法2：使用 String 的 push_str 方法
    // let mut s = input.to_string();
    // s.push_str(" world!");
    // s

    // 方法3：使用 + 运算符
    // input.to_string() + " world!"
}

fn replace_me(input: &str) -> String {
    // replace 方法替换字符串中的内容
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
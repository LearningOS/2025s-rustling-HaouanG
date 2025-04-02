pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // 处理 Uppercase
    fn uppercase(s: &str) -> String {
        s.to_uppercase()
    }

    // 处理 Trim
    fn trim(s: &str) -> String {
        s.trim().to_string()
    }

    // 处理 Append
    fn append(s: &str, times: usize) -> String {
        let mut result = s.to_string();
        for _ in 0..times {
            result.push_str("bar");
        }
        result
    }

    // transformer 函数，根据命令转换字符串
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![];

        for (string, command) in input.iter() {
            let transformed = match command {
                Command::Uppercase => uppercase(string),
                Command::Trim => trim(string),
                Command::Append(n) => append(string, *n),
            };
            output.push(transformed);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    // 导入 transformer 函数
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);

        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}

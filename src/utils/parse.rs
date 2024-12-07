pub fn parse_commands(line: &str) -> Vec<String> {
    let mut result = Vec::new(); // 用于存储结果
    let mut current_token = String::new(); // 当前正在构建的命令或参数
    let mut in_single_quote = false; // 是否在单引号内
    let mut in_double_quote = false; // 是否在双引号内
    let mut escaping = false; // 是否正在处理转义字符

    let chars = line.chars().peekable(); // 逐字符遍历

    for ch in chars {
        match ch {
            '\\' if !escaping => escaping = true, // 处理转义符
            '"' if !in_single_quote => {
                if in_double_quote {
                    result.push(current_token.drain(..).collect()); // 结束双引号，保存当前token
                }
                in_double_quote = !in_double_quote;
            }
            '\'' if !in_double_quote => {
                if in_single_quote {
                    result.push(current_token.drain(..).collect()); // 结束单引号，保存当前token
                }
                in_single_quote = !in_single_quote;
            }
            ch if escaping => {
                current_token.push(ch); // 转义字符后直接加入当前token
                escaping = false;
            }
            ch if ch.is_whitespace() && !in_single_quote && !in_double_quote => {
                if !current_token.is_empty() {
                    result.push(current_token.drain(..).collect()); // 空格分隔参数
                }
            }
            ch => current_token.push(ch), // 其他字符加入当前token
        }
    }

    // 如果最后有残留的内容，加入结果
    if !current_token.is_empty() {
        result.push(current_token);
    }

    result
}

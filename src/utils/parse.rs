pub fn parse_commands(line: &str) -> Vec<String> {
    let mut result = Vec::new(); // 用于存储结果
    let mut current_token = String::new(); // 当前正在构建的命令或参数
    let mut in_single_quote = false; // 是否在单引号内
    let mut in_double_quote = false; // 是否在双引号内
    let mut escaping = false; // 是否正在处理转义字符

    let mut chars = line.chars().peekable(); // 逐字符遍历

    while let Some(ch) = chars.next() {
        if escaping {
            // 如果是转义状态，当前字符直接加入当前token
            current_token.push(ch);
            escaping = false;
        } else if ch == '\\' {
            // 在引号外部，反斜杠视为转义字符
            if in_double_quote {
                // 只有在双引号内，且反斜杠后面是特定字符时，反斜杠才有效
                if let Some(next_ch) = chars.peek() {
                    if *next_ch == '\\' || *next_ch == '"' || *next_ch == '$' || *next_ch == '\n' {
                        escaping = true; // 设置转义标志
                    } else {
                        current_token.push(ch); // 如果不是特殊字符，反斜杠作为普通字符处理
                    }
                } else {
                    current_token.push(ch); // 如果是最后一个字符，反斜杠作为普通字符处理
                }
            } else {
                escaping = true; // 反斜杠在引号外部，标记为转义状态
            }
        } else if ch == '"' && !in_single_quote {
            // 处理双引号
            if in_double_quote {
                result.push(current_token.clone()); // 结束双引号时，把当前token加入结果
                current_token.clear();
            }
            in_double_quote = !in_double_quote;
        } else if ch == '\'' && !in_double_quote {
            // 处理单引号
            if in_single_quote {
                result.push(current_token.clone()); // 结束单引号时，把当前token加入结果
                current_token.clear();
            }
            in_single_quote = !in_single_quote;
        } else if ch.is_whitespace() && !in_single_quote && !in_double_quote {
            // 空格分隔参数，只有在不在引号中的情况下才有效
            if !current_token.is_empty() {
                result.push(current_token.clone());
                current_token.clear();
            }
        } else {
            // 其他字符加入当前token
            current_token.push(ch);
        }
    }

    // 如果最后有残留的内容，加入结果
    if !current_token.is_empty() {
        result.push(current_token);
    }

    result
}

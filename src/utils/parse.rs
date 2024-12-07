pub fn parse_commands(line: &str) -> Vec<String> {
    let mut result = Vec::new(); // 用于存储结果
    let mut current_token = String::new(); // 当前正在构建的命令或参数
    let mut in_double_quote = false; // 是否在双引号内
    let mut escaping = false; // 是否正在处理转义字符

    let chars = line.chars().peekable(); // 逐字符遍历

    for ch in chars {
        if escaping {
            // 如果前一个字符是反斜杠，跳过转义，当前字符加入到token中
            current_token.push(ch);
            escaping = false;
        } else if ch == '\\' {
            // 如果当前字符是反斜杠，设置转义标志
            escaping = true;
        } else if ch == '"' {
            // 处理双引号
            if in_double_quote {
                // 如果已经在双引号内，遇到闭合双引号时将当前内容作为一个整体
                if ch.is_whitespace() {
                    result.push(current_token.clone());
                    current_token.clear();
                }
            }
            // 切换双引号内外状态
            in_double_quote = !in_double_quote;
        } else if ch.is_whitespace() && !in_double_quote {
            // 空格分隔参数，只有在不在双引号中的情况下才有效
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

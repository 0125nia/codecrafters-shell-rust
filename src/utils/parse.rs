pub fn parse_commands(line: &str) -> Vec<String> {
    // 创建一个字符迭代器，去掉两端的空格
    let mut s_iter = line.trim().chars().peekable();
    // 当前命令的字符串（用于存储每个分割出的命令）
    let mut cur_s = String::new();
    // 用于存储解析后的命令字符串
    let mut ret = Vec::new();
    // 标记是否在单引号内
    let mut in_single_quote = false;
    // 标记是否在双引号内
    let mut in_double_quote = false;

    // 遍历输入的每个字符
    while let Some(c) = s_iter.next() {
        // 如果遇到单引号且不在双引号内，则切换单引号状态
        if c == '\'' && !in_double_quote {
            in_single_quote = !in_single_quote;
        // 如果遇到双引号且不在单引号内，则切换双引号状态
        } else if c == '"' && !in_single_quote {
            in_double_quote = !in_double_quote;
        // 处理普通的转义字符（如：\n, \\）
        } else if c == '\\' && !in_single_quote && !in_double_quote {
            // 读取转义符后的下一个字符并将其添加到当前命令字符串
            let c = s_iter.next().unwrap();
            cur_s.push(c);
        // 处理双引号中的转义字符（如：\" 或 \$）
        } else if c == '\\' && in_double_quote {
            // 根据转义符后面字符的不同，决定是否需要转义
            match s_iter.peek().unwrap() {
                '\\' | '$' | '"' => {
                    cur_s.push(s_iter.next().unwrap());
                }
                // 如果遇到其他字符，保留原样
                _ => cur_s.push(c),
            };
        // 如果遇到空格且不在引号内，认为是命令的分隔符
        } else if c == ' ' && !in_single_quote && !in_double_quote {
            // 如果当前命令不为空，则将其加入返回结果中
            if !cur_s.is_empty() {
                ret.push(cur_s);
                cur_s = String::new();
            }
        // 如果是普通字符，添加到当前命令字符串中
        } else {
            cur_s.push(c);
        }
    }

    // 如果最后一个命令字符串非空，添加到结果中
    if !cur_s.is_empty() {
        ret.push(cur_s);
    }

    // 返回解析后的命令列表
    ret
}

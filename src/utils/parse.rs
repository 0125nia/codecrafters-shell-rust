pub fn parse_commands(line: &str) -> Vec<String> {
    let mut s_iter = line.trim().chars().peekable();
    let mut cur_s = String::new();
    let mut ret = Vec::new();
    let mut in_single_quote = false;
    let mut in_double_quote = false;

    while let Some(c) = s_iter.next() {
        if c == '\'' && !in_double_quote {
            in_single_quote = !in_single_quote;
        } else if c == '"' && !in_single_quote {
            in_double_quote = !in_double_quote;
        } else if c == '\\' && !in_single_quote && !in_double_quote {
            let c = s_iter.next().unwrap();
            cur_s.push(c);
        } else if c == '\\' && in_double_quote {
            match s_iter.peek().unwrap() {
                '\\' | '$' | '"' => {
                    cur_s.push(s_iter.next().unwrap());
                }
                _ => cur_s.push(c),
            };
        } else if c == ' ' && !in_single_quote && !in_double_quote {
            if !cur_s.is_empty() {
                ret.push(cur_s);
                cur_s = String::new();
            }
        } else {
            cur_s.push(c);
        }
    }
    if !cur_s.is_empty() {
        ret.push(cur_s);
    }
    ret
}

use data_structures::stack::Stack;

const OPENS: &str = "([{";
const CLOSES: &str = ")]}";

fn par_match(open: char, close: char) -> bool {
    OPENS.find(open) == CLOSES.find(close)
}

pub fn par_checker(par: &str) -> bool {
    let mut stack = Stack::new();

    for c in par.chars() {
        match c {
            open if OPENS.contains(open) => {
                stack.push(open);
            }
            close if CLOSES.contains(close) => {
                if let Some(open) = stack.pop() {
                    if !par_match(open, close) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}

pub fn run_par_checker() {
    // 匹配单一括号
    assert_eq!(par_checker("((()))"), true);
    assert_eq!(par_checker("()()"), true);
    assert_eq!(par_checker("(()(()))"), true);
    assert_eq!(par_checker("((())"), false);
    assert_eq!(par_checker("(()))"), false);
    assert_eq!(par_checker("(((()())"), false);

    // 匹配混合括号
    assert_eq!(par_checker("()[]{}"), true);
    assert_eq!(par_checker("{[()]}"), true);
    assert_eq!(par_checker("{[()(]]"), false);

    // 匹配夹杂其他字符
    assert_eq!(par_checker("(2+3){func}[abc]"), true);
    assert_eq!(par_checker("(2+3)*(3-1"), false);
}

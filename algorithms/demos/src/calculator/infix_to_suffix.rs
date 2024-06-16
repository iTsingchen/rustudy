use data_structures::stack::Stack;

fn precedence(op: char) -> i32 {
    match op {
        '*' | '/' => 2,
        '+' | '-' => 1,
        _ => 0,
    }
}

pub fn infix_to_suffix(infix: &[char]) -> Vec<char> {
    let mut suffix = Vec::new();
    let mut op_stack = Stack::new();

    for &c in infix.iter() {
        if c.is_digit(10) {
            suffix.push(c);
        } else if c == '(' {
            op_stack.push(c);
        } else if c == ')' {
            while let Some(op) = op_stack.pop() {
                if op == '(' {
                    break;
                }
                suffix.push(op)
            }
        } else {
            while (!op_stack.is_empty()) && (precedence(*op_stack.peek().unwrap()) >= precedence(c))
            {
                suffix.push(op_stack.pop().unwrap())
            }
            op_stack.push(c)
        }
    }

    suffix.into_iter().chain(op_stack.info_iter()).collect()
}

#[test]
fn simple() {
    assert_eq!(infix_to_suffix(&['1', '+', '2']), vec!['1', '2', '+']);
    assert_eq!(
        infix_to_suffix(&['1', '+', '2', '-', '3']),
        vec!['1', '2', '+', '3', '-']
    );
    assert_eq!(
        infix_to_suffix(&['1', '+', '2', '/', '3']),
        vec!['1', '2', '3', '/', '+']
    )
}

#[test]
fn bracket() {
    assert_eq!(
        infix_to_suffix(&['1', '+', '(', '2', '-', '3', ')']),
        vec!['1', '2', '3', '-', '+']
    );
}

use data_structures::stack::Stack;

fn do_calc(op: char, n1: i32, n2: i32) -> i32 {
    match op {
        '+' => n1 + n2,
        '-' => n1 - n2,
        '*' => n1 * n2,
        '/' if n2 != 0 => n1 / n2,
        '/' if n2 == 0 => panic!("ZeroDivisionError: Invalid operation!"),
        _ => panic!("OperatoreError: Invalid operator: {:?}", op),
    }
}

pub fn suffix_eval(suffix: Vec<char>) -> i32 {
    let mut stack = Stack::new();

    for c in suffix {
        if c.is_digit(10) {
            stack.push(c.to_digit(10).unwrap() as i32)
        } else {
            let n2 = stack.pop().unwrap();
            let n1 = stack.pop().unwrap();
            stack.push(do_calc(c, n1, n2))
        }
    }

    stack.pop().unwrap()
}

#[test]
fn suffix_eval_test() {
    assert_eq!(suffix_eval(vec!['1', '2', '+']), 3)
}

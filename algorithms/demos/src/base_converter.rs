use data_structures::stack::Stack;

fn base_converter(mut dec_num: u32, base: u32) -> String {
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    let mut stack = Stack::new();

    while dec_num > 0 {
        let rem = dec_num % base;
        stack.push(rem);
        dec_num /= base;
    }

    stack
        .info_iter()
        .map(|n| digits[n as usize].to_string())
        .collect::<String>()
}

pub fn run_base_converter() {
    assert_eq!(base_converter(10, 2), "1010");
    assert_eq!(base_converter(43, 16), "2B");
}

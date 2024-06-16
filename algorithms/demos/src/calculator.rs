mod infix_to_suffix;
mod suffix_eval;

use super::par_checker::par_checker;
use infix_to_suffix::infix_to_suffix;
use suffix_eval::suffix_eval;

fn calculat(expr: &str) -> Option<i32> {
    if !par_checker(expr) {
        return None;
    }

    let chars = expr
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>();
    let suffix = infix_to_suffix(&chars);
    Some(suffix_eval(suffix))
}

pub fn run_calculator() {
    assert_eq!(calculat("("), None);
    assert_eq!(calculat("1+1").unwrap(), 2);
    assert_eq!(calculat("1+2*3").unwrap(), 7);
    assert_eq!(calculat("(1+2)*3").unwrap(), 9);
    assert_eq!(calculat("(1+2)*(6-4)").unwrap(), 6);
    assert_eq!(calculat("((1+2)*(6-4))/3").unwrap(), 2);
}

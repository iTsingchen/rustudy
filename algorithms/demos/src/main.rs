mod par_checker;
use par_checker::run_par_checker;

mod base_converter;
use base_converter::run_base_converter;

mod calculator;
use calculator::run_calculator;

fn main() {
    run_par_checker();
    run_base_converter();
    run_calculator()
}

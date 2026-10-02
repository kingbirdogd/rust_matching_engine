use common;
use math;
use utils;

fn main() {
    common::log_info("Starting application");
    utils::print_separator();
    
    let result1 = math::add(10, 5);
    println!("{}", utils::format_result("Addition", result1));
    
    let result2 = math::multiply(3, 7);
    println!("{}", utils::format_result("Multiplication", result2));
    
    utils::print_separator();
    common::log_info("Application complete");
}

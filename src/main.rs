use colored::*;
use std::env;
use Even_odd_checker::even_odd_checker;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let msg = "No arguments found !".red();
        panic!("{}", msg);
    }

    let num = args[1].trim().parse().unwrap_or_else(|err| {
        let msg = "invalid number !".red();
        panic!("{} {}", msg, err);
    });

    let result = even_odd_checker::check_even_odd(num);

    let mut msg = "Wow! Your number is".green();
    println!("{} {}", msg, result.green());
}

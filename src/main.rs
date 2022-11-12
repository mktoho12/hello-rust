mod hacker_rank;
mod book;
mod masa_u;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "diagonal_difference" => hacker_rank::diagonal_difference::main(),
        "plus_minus" => hacker_rank::plus_minus::main(),
        "staircase" => hacker_rank::staircase::main(),
        "mini_max_sum" => hacker_rank::mini_max_sum::main(),
        "birthday_cake_candles" => hacker_rank::birthday_cake_candles::main(),
        "time_conversion" => hacker_rank::time_conversion::main(),
        "grading" => hacker_rank::grading::main(),

        "book/struct" => book::my_struct::main(),
        "book/enum" => book::my_enum::main(),

        "masa_u/digits" => masa_u::digits::main(),
        
        _ => println!("Please specify problem name."),
    }
}

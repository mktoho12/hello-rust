
// const RADIX: u32 = 10;

// pub fn main() {
//     let stdin = io::stdin();

//     // １行読み飛ばす
//     let mut digit = String::new();
//     stdin.read_line(&mut digit);

//     let digits = digit.trim().chars();
//     let ints = digits.map(|digit| digit.to_digit(RADIX).unwrap()).collect::<Vec<u32>>();
//     println!("{}{}", ints[0] + ints[3], ints[1] + ints[2]);
// }

// use std::io;

// pub fn main() {
//     let stdin = io::stdin();
    
//     let mut digit = String::new();
//     _ = stdin.read_line(&mut digit);
    
//     let digits = &digit.trim().split("").collect::<Vec<_>>()[1..4];
//     let ints = digits.iter().map(|&digit| digit.parse::<i32>().unwrap()).collect::<Vec<i32>>();
//     println!("{}{}", ints[0] + ints[3], ints[1] + ints[2]);
// }


use std::io::{stdin, Error, Stdin, BufRead};

fn ctoi(c: char) -> u32 {
    c.to_digit(10).unwrap()
}

pub fn main() {
    let input = &read_lines(stdin())[0];
    let nums: Vec<_> = input.chars().map(ctoi).collect();

    println!("{}{}", nums[0] + nums[3], nums[1] + nums[2]);
}

fn ok<T>(result: Result<T, Error>) -> T {
    result.unwrap()
}

pub fn read_lines(stdin: Stdin) -> Vec<String> {
    stdin.lock().lines().map(ok).collect()
}

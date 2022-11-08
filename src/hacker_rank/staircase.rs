use std::io::{self, BufRead};

/*
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn step(n: i32, max: i32) -> String {
    let mut result: Vec<char> = vec![];

    for _ in 0..max - n {
        result.push(' ');
    }

    for _ in 0..n {
        result.push('#');
    }

    result.iter().collect::<String>()
}

fn staircase(n: i32) {
    for m in 1..=n {
        println!("{}", step(m, n));
    }
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    staircase(n);
}

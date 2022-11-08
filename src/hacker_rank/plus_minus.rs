use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn divide(dividend: usize, divisor: usize) -> f32 {
    dividend as f32 / divisor as f32
}

 fn plus_minus(arr: &[i32]) {
    if arr.len() == 0 {
        println!("0\n0\n0\n");
        return;
    }

    let posi_count = arr.iter().filter(|n| **n > 0).count();
    let zero_count = arr.iter().filter(|n| **n == 0).count();
    let nega_count = arr.iter().filter(|n| **n < 0).count();

    let posi_rate = divide(posi_count, arr.len());
    let zero_rate = divide(zero_count, arr.len());
    let nega_rate = divide(nega_count, arr.len());

    println!("{:.6}", posi_rate);
    println!("{:.6}", nega_rate);
    println!("{:.6}", zero_rate);
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plus_minus(&arr);
}

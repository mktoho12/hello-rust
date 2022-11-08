use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 * 
 * https://www.hackerrank.com/challenges/mini-max-sum/problem?isFullScreen=true
 */

fn mini_max_sum(arr: &[u64]) {
  let max = arr.iter().max().unwrap();
  let min = arr.iter().min().unwrap();

  let sum = &arr.iter().sum::<u64>();

  println!("{} {}", sum - max, sum - min);
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<u64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<u64>().unwrap())
        .collect();

    mini_max_sum(&arr);
}

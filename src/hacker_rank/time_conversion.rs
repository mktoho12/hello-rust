use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 *
 * https://www.hackerrank.com/challenges/time-conversion/problem?isFullScreen=true
 */

fn time_conversion(s: &str) -> String {
    let (hour, minute, second, ampm) = (&s[..2], &s[3..5], &s[6..8], &s[8..]);
    let hour = hour.parse::<i32>().ok().unwrap();

    let hour = match (ampm, hour) {
        ("AM", 12) => 0,
        ("PM", 12) => 12,
        ("PM", _) => hour + 12,
        _ => hour,
    };

    String::from(format!("{:02}:{minute}:{second}", hour))
}

pub fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = time_conversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}

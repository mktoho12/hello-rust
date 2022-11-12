#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn print_option(option: Option<i32>) {
    let text = match option {
        None => String::from("None"),
        Some(v) => format!("{v}"),
    };

    println!("{text}")
}

pub fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}, {:?}", home, loopback);

    let none = None::<i32>;
    let some = Some(42);

    print_option(none);
    print_option(some);

    println!("{}", ctoi('8'))
}

pub fn ctoi(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("Not a Number!!!!! Go to the hell!!")
    }
}
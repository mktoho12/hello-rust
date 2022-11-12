#[derive(Debug)]
struct Rectangle {
  width: u64,
  height: u64,
}

impl Rectangle {
  fn area(&self) -> u64 {
    self.width * self.height
  }
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

// fn value_in_cents(coin: Coin) -> u8 {
//   match coin {
//     Penny
//   }
// }

pub fn main() {
  let scale = 2;
  let rect = Rectangle {
    width: 30 * scale,
    height: 40,
  };

  dbg!(rect.area());
}
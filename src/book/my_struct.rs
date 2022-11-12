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

pub fn main() {
  let scale = 2;
  let rect = Rectangle {
    width: 30 * scale,
    height: 40,
  };

  dbg!(rect.area());
}
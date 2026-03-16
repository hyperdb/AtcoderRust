use proconio::input;

fn main() {
  input! {
    a:usize,
    b:usize,
    h:usize,
  }
  // 台形の面積を求める
  let result: usize = (a + b) * h / 2;
  println!("{}", result);
}

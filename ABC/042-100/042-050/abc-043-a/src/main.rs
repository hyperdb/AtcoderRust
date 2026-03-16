use proconio::input;

fn main() {
  input! {
      n:usize
  }
  // 等差数列の和の公式を使って、1からnまでの整数の和を求める
  let r: usize = n * (n + 1) / 2;

  println!("{}", r);
}

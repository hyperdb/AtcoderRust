use proconio::input;

fn main() {
  input! {
    n:usize,
    k:usize,
    x:usize,
    y:usize,
  }
  // 宿泊数 n < 切替泊数 k の場合は全て x円
  // そうでない場合は k 泊までが x円で残りが y円
  println!("{}", if n < k { n * x } else { k * x + (n - k) * y });
}

use proconio::input;

fn main() {
  input! {
      n:usize
  }

  let r: usize = n * (n + 1) / 2;

  println!("{}", r);
}

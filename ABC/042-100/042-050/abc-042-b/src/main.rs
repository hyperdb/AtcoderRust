use proconio::input;

fn main() {
  input! {
      n:usize,
      _l:usize,
      mut s: [String; n]
  }
  // ソートして結合する
  s.sort();

  println!("{}", s.concat())
}

use proconio::input;

fn main() {
  input! {
    mut a:[usize; 3],
  }

  // ソートして、5, 5, 7 の順番になるか確認する
  a.sort();

  let mut r: bool = false;
  if a.eq(&[5, 5, 7]) {
    r = true;
  }
  let ans = if r { "YES" } else { "NO" };

  println!("{}", ans);
}

use proconio::input;

fn main() {
  input! {
      mut a:[usize; 3],
  }
  a.sort();

  let mut r: bool = false;
  if a.eq(&[5, 5, 7]) {
    r = true;
  }
  let ans = if r { "YES" } else { "NO" };

  println!("{}", ans);
}

use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
      s: Chars
  }
  let mut buf: Vec<char> = vec![];
  // 文字列を左から順に処理していく
  for c in s {
    // 文字が 'B' の場合は buf の末尾を削除する
    if c == 'B' {
      buf.pop();
    } else {
      buf.push(c);
    }
  }
  // buf の内容を文字列に変換して出力
  let cs: String = buf.iter().collect();
  println!("{}", &cs);
}

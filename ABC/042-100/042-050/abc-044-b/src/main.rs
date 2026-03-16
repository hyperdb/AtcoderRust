use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    s:String,
  }
  // 文字の出現回数を数える
  let mut map = HashMap::new();
  s.chars().for_each(|c| {
    *map.entry(c).or_insert(0) += 1;
  });
  // 出現回数が奇数の文字があるかどうかをチェック
  let mut found_odd = false;
  for v in map.values() {
    if v % 2 != 0 {
      found_odd = true;
      break;
    }
  }
  // 出現回数が奇数の文字がある場合は "No"、そうでない場合は "Yes" を出力
  println!("{}", if found_odd { "No" } else { "Yes" });
}

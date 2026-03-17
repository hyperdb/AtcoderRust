use proconio::input;

fn main() {
  input! {
    n:usize,
    a:usize,
    x:[usize;n],
  }
  // DPテーブル: dp[i][j][k] := i番目までの整数から、合計がjで、k個選ぶ方法の数
  let mut dp = vec![vec![vec![0i64; n + 1]; n * 50 + 1]; n + 1];
  // ベースケース: 0個選んで合計0の方法は1通り（何も選ばない）
  dp[0][0][0] = 1;
  // DPの遷移
  for i in 0..n {
    // jは合計、kは選んだ個数
    for j in 0..x.iter().sum::<usize>() {
      for k in 0..n {
        // dp[i][j][k]が0の場合はスキップ
        if dp[i][j][k] == 0 {
          continue;
        }

        dp[i + 1][j][k] += dp[i][j][k];
        dp[i + 1][j + x[i]][k + 1] += dp[i][j][k];
      }
    }
  }

  // 答えを求める
  let mut r: i64 = 0;
  for k in 1..n + 1 {
    r += dp[n][a * k][k];
  }

  println!("{}", r);
}

use proconio::input;

fn main() {
  input! {
      n:usize
  }

  if n == 0 {
    println!("{}", n);
  } else {
    input! {
        a:[i32;n],
    }
    // 平均値を求める
    let sum = a.iter().sum::<i32>();
    let ave = sum as f32 / a.len() as f32;

    let mut cost = 0;
    // 平均値が整数の場合はそのまま計算
    if ave % 1.0 == 0.0 {
      for i in a {
        cost += (i - ave as i32).pow(2);
      }
    } else {
      // 平均値が整数でない場合は切り上げと切り捨ての両方を試して小さい方を採用
      let mut cost_h = 0;
      let mut cost_l = 0;
      let ave_h = ave.ceil() as i32;
      let ave_l = ave.floor() as i32;

      for i in a {
        cost_h += (i - ave_h).pow(2);
        cost_l += (i - ave_l).pow(2);
      }
      cost = std::cmp::min(cost_h, cost_l);
    }
    println!("{}", cost);
  }
}

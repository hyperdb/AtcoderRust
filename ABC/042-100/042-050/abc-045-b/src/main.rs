use proconio::input;

fn main() {
  input! {
    sa:String,
    sb:String,
    sc:String,
  }
  // それぞれのプレイヤーのカードをスタックとして管理する
  let sa_cards = sa.chars().collect::<Vec<char>>();
  let sb_cards = sb.chars().collect::<Vec<char>>();
  let sc_cards = sc.chars().collect::<Vec<char>>();
  let mut cards = vec![sa_cards, sb_cards, sc_cards];
  // プレイヤーの名前を配列で管理する
  const PLAYER_NAMES: [&str; 3] = ["A", "B", "C"];
  // 初期プレイヤーは A
  let mut player: usize = 0;
  // カードが無くなるまでループ
  loop {
    if cards[player].len() == 0 {
      // プレイヤーのカードが無くなったら終了
      println!("{}", PLAYER_NAMES[player]);
      break;
    }
    // 次のプレイヤーを決める
    let card = cards[player].remove(0);
    player = match card {
      'a' => 0,
      'b' => 1,
      'c' => 2,
      _ => unreachable!(),
    };
  }
}

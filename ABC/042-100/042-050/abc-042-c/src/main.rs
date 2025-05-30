use proconio::input;

fn main() {
  input! {
      n:usize,
      k:usize,
      d:[char; k]
  }

  for i in n..100000 {
    let s: String = i.to_string();
    let mut r: bool = true;
    for j in 0..s.len() {
      let c: char = s.chars().nth(j).unwrap();
      if d.iter().find(|&x| *x == c) != None {
        r = false;
        break;
      }
    }
    if r {
      println!("{}", i);
      break;
    }
  }
}

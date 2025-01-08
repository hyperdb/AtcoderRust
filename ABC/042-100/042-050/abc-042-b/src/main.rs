use proconio::input;

fn main() {

    input! {
        n:usize,
        _l:usize,
        mut s: [String; n]
    }

    s.sort();

    println!("{}", s.concat())
}

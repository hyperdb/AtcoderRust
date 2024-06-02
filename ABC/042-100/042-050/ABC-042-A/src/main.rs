use proconio::input;

fn main() {

    input! {
        mut a:[usize; 3],
    }

    a.sort();

    let mut r : bool = false;
    let t: usize = a.iter().sum();

    if t == 17
    {
        if a[0] == 5 && a[1] == 5
        {
            r = true;
        }
    }

    let ans = if r {"YES"} else {"NO"};

    println!("{}", ans);
}

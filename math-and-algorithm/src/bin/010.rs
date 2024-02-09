use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let ans = recursion(n);
    println!("{}", ans);
}

fn recursion (n: usize) -> usize {
    if n == 0 {return 1;}
    else {
        return n * recursion(n - 1);
    }
}
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let t = a.iter().filter(|&&x| x >= m).count();
    println!("{}", t);
}

use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
    }
    let mut count: i32 =0;
    for i in 1..n+1 {
        for j in 1..n+1 {
            if i + j <= s {
                count += 1;
            }
        }
    }
    println!("{}",count);
}

use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
				x: usize,
				y: usize,
    }
		let mut count: i32 = 0;
		for i in 1..n+1{
			if i % x == 0 || i % y == 0{
				count += 1;
			}
		}
		println!("{}", count);
}

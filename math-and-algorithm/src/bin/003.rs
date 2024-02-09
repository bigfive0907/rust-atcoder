use proconio::{fastout, input};
#[fastout]
fn main() {
	input! {
			n:usize,
			a:[u32;n],
	}
	println!("{}",a.iter().sum::<u32>());
}

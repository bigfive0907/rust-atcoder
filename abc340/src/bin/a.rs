use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};
[#fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let mut vec: Vec<usize> = Vec::new();
    let diff = (b-a)/c;
    for i in 0..diff+1 {
        vec.push(i * c + a);
    }
    vec.iter().for_each(|&x| print!("{} ", x));

    println!();
}

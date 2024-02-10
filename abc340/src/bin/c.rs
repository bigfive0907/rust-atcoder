use proconio::{
    fastout,
    input,
    marker::{Bytes, Chars},
};
fn counting (n:i64,count:i64) -> i64 {
    if n < 2 {
        return count;
    }else{
        return counting((n+1)/2,count+1);
    }
}
#[fastout]
fn main() {
    //3
    let mut count:i64=0;
    input! {
        n: i64,
    }
    let c = counting(n,count);
    //c=2
    let p = 1 << c;//2^count
    let q = p-n;
    println!("{}",c*n-q);

}

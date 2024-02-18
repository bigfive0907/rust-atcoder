#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_macros)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use num_traits::ToPrimitive;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let mut vec: Vec<usize> = Vec::new();
    let mut arr = [
	['a','b','c'],
	['d','e','f'],
	['g','h','i'],
	['j','k','m'],
    ];

    let diff = (b-a)/c;
    for i in 0..diff+1 {
        vec.push(i * c + a);
    }
    vec.iter().for_each(|&x| print!("{} ", x));

    for ans in vec {
        print!("{} ", ans);
    }
    for ans in arr {
        ans.iter().for_each(|&x| print!("{} ", x));
    }
    println!();

}

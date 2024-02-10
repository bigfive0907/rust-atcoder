#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_macros)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use proconio::{
    fastout,
    input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        q:usize,
    }
    let mut v = vec![];

    for _ in 0..q {
        input! {
            t:usize,
        }
        if t == 1 {
            input! {
                x:usize,
            }
            v.push(x);
        }
        if t == 2 {
            input! {
                k:usize,
            }
            println!("{}", v[v.len() - k]);
        }
    }
}

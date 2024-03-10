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
use proconio::{
    fastout,
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
    source::line::LineSource,
};
use std::{
    cmp::{Reverse,min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    f64::{self, consts::PI},
    io::{self, BufReader},
    iter::FromIterator,
    mem::swap,
};
const MOD: usize = 1_000_000_007;
const INF: isize = 2_000_000_000;
pub type Graph = Vec<Vec<usize>>;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        h: [isize; n],
    }
    let mut dp = vec![INF; n];
    dp[0] = 0;
    for i in 1..n {
        if i == 1 {
            dp[i] = (h[i] - h[i - 1]).abs();
        }
        for j in 1..=k {
            if i < j {
                break;
            }
            dp[i] = min(dp[i], dp[i-j]+(h[i]-h[i-j]).abs());
        }
    }
    println!("{}", dp[n-1]);
}

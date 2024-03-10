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
    cmp::{min,max,Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    f64::{self, consts::PI},
    io::{self, BufReader},
    iter::FromIterator,
    mem::swap,
};
const MOD: usize = 1_000_000_007;
const INF: usize = 2_000_000_000;
pub type Graph = Vec<Vec<usize>>;

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize);n],
    }
    let max_v = 1000 * 100 + 10;
    let mut dp = vec![vec![INF;max_v+1];n+1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..max_v{
            // 価値がjの時の重さの最小値を求める
            if j >= wv[i].1{
                dp[i+1][j] = min(dp[i][j], wv[i].0 + dp[i][j-wv[i].1]);
            }
            else {
                dp[i+1][j] = dp[i][j];
            }
        }
    }
    let mut res = max_v;
    for i in 0..=max_v {
        if dp[n][i] <= w {
            res = i;
        }
    }
    println!("{}", res);
}

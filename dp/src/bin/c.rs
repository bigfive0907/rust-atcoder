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
    cmp::{max,min,Reverse},
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
        arr: [[isize; 3];n],
    }
    let mut dp = vec![[0;3];n];
    for i in 0..3 {
        dp[0][i] = arr[0][i];
    }
    for k in 1..n {
        for i in 0..3 {
            for j in 0..3 {
                //同じ番号は選べないのでスキップ
                if i == j {
                    continue;
                }
                // dp[k][i]にはその時点までの最大値を保存しておく(ex. i=1なら0->2の順で0とdp[k][0],dp[k][0]とdp[k][2] )。k-1までの幸福度(j=同じ番号以外の0~2)とkのi番目の幸福度の和の最大値を求める
                dp[k][i] = max(dp[k][i],arr[k][i]+dp[k-1][j]);
            }
        }
    }
    println!("{}",max(dp[n-1][0],max(dp[n-1][1],dp[n-1][2])));
}

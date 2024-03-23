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
    cmp::{Reverse,min,max},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    f64::{self, consts::PI},
    io::{self, BufReader},
    iter::FromIterator,
    mem::swap,
};
use regex::Regex;

const MOD: usize = 1_000_000_007;
const INF: isize = 2_000_000_000;
pub type Graph = Vec<Vec<usize>>;

#[fastout]
fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n]
    }
    //kのsum - a[n]
    let mut sum = (k*(k+1))/2;
    let mut set: HashSet<usize> = HashSet::new();
    
    for i in 0..n {
        if a[i] <= k && !set.contains(&a[i]) {
            sum -= a[i];
            set.insert(a[i]);
        }
    }
    println!("{}",sum);
}

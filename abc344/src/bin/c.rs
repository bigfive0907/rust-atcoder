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
    cmp::Reverse,
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
        a:[usize;n],
        m: usize,
        b:[usize;m],
        l: usize,
        c:[usize;l],
        q: usize,
        x:[usize;q],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        for j in 0..m {
            for k in 0..l {
                map.insert(a[i]+b[j]+c[k], 1);
            }
        }
    }
    for i in 0..q {
        if map.contains_key(&x[i]) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

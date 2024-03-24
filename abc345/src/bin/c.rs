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
const MOD: usize = 1_000_000_007;
const INF: isize = 2_000_000_000;
pub type Graph = Vec<Vec<usize>>;

#[fastout]
fn main() {
    input! {
        s:Chars,
    }
    let mut map:HashMap<char, usize> = HashMap::new();
    let mut ans = s.len()*s.len()+1;
    let mut flag = false;
    for i in 0..s.len() {
        *map.entry(s[i]).or_insert(0) += 1;
    }
    for k in &map {
        if k.1 > &1 {
            flag = true;
        }
        ans -= k.1*k.1;
    }
    ans /= 2;
    if flag {
        ans += 1;
    }
    println!("{}",ans);
}

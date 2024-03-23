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
        n: usize,
        mut s: Chars,
        c:[usize;n],
    }
    let mut cost = 0;
    let mut max_cost = 0;
    let mut max_cost_index = 0;

    for mut i in 0..n-1 {
        if s[i] == s[i+1] {
            if c[i] < c[i+1] {
                cost += c[i];
                if c[i] > max_cost {
                    max_cost = c[i];
                    max_cost_index = i;
                }
                if s[i] == '1' {
                    s[i] = '0';
                }
                else {
                    s[i] = '1';
                }
            }
            else {
                cost += c[i+1];
                if c[i+1] > max_cost {
                        max_cost = c[i+1];
                        max_cost_index = i+1;
                    }
                if s[i+1] == '1' {
                    s[i+1] = '0';
                }
                else {
                    s[i+1] = '1';
                }
                i += 1;
            }
        }
    }

    cost -= max_cost;
    if s[max_cost_index] == '1' {
        s[max_cost_index] = '0';
    }
    else {
        s[max_cost_index] = '1';
    }

    println!("{}",cost);
    

}

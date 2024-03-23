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
        w:usize,
        b:usize,
    }
    let mut s:&str =  "wbwbwwbwbwbw";
    let mut st:String = s.to_string();
    for i in 0..10 {
        st = format!("{}{}", st, st);
        //println!("{}",st);
    }
    let mut ans = st.clone();

    for i in 0..st.len()-w-b {
        let mut w_count = 0;
        let mut b_count = 0;
        for j in i..w+b+i {
            if ans.chars().nth(j).unwrap() == 'w' {
                w_count += 1;
            }
            if ans.chars().nth(j).unwrap() == 'b'  {
                b_count += 1;
            }
            if w_count == w && b_count == b {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

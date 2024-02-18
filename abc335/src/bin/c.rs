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

//use acl_dsu::Dsu;
//use acl_fenwicktree::FenwickTree;
//use acl_lazysegtree::LazySegtree;
//use acl_modint::{ModInt1000000007, ModInt998244353};
//use acl_segtree::{Additive, Max, Min, Multiplicative, Segtree};
//use my_atcoder_library::*;

use fixedbitset::FixedBitSet;
use itertools::Itertools;

use num_integer::*;
use num_traits::clamp;
use permutohedron::factorial;
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
use superslice::Ext;

const MOD: usize = 1_000_000_007;
const INF: isize = 2_000_000_000;

pub type Graph = Vec<Vec<usize>>;

#[cfg(target_pointer_width = "64")]
pub type fsize = f64;
#[cfg(target_pointer_width = "32")]
pub type fsize = f32;

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

#[fastout]
fn main() {
    input! {
        n: u64,
        q: u64,
    }
    let mut dragon: VecDeque<(i64, i64)> = VecDeque::new();
    for i in 1..=n {
        dragon.push_back((i as i64, 0));
        
    }
    for _ in 0..q {
        input! {
            num: u64,
        }
        if num == 1 {
            input! {
                t: char,
            }
            let now = dragon.get(0).cloned().unwrap();
            match t {
                'L' => {
                    dragon.push_front((now.0 -1 , now.1, ));
                }
                'R' => {
                    dragon.push_front((now.0 +1 , now.1, ));
                }
                'U' => {
                    dragon.push_front((now.0, now.1 + 1, ));
                }
                'D' => {
                    dragon.push_front((now.0, now.1 - 1, ));
                }
                _ => {
                    panic!("error");
                }
            }
            dragon.pop_back();
        }
        else if num == 2 {
            input! {
                t: i64,
            }
            let val = dragon.get(t as usize - 1).unwrap();
            println!("{} {}", val.0, val.1);
        }
    }
}

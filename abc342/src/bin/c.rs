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
  fastout, input,
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
      n: i64,
      s: Chars,
      q: i64,
  }
  let mut vec = vec!['0'; 200];
  let mut ans = "".to_string();
  for i in 0..q {
    input! {
        a: char,
        b: char,
    }
    for j in 0..200 {
      if vec[j] == a {
        vec[j] = b;
      }
    }
     if vec[a as usize] == '0' {
        vec[a as usize] = b;
     }
  }
  for i in 0..s.len() {
    if vec[s[i] as usize] == '0' {
      ans.push(s[i]);
    }
    else {
        ans.push(vec[s[i] as usize]);
    }
  }
  println!("{}", ans);
}
